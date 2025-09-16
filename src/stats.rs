use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use rusqlite::{Connection, Result};
use log::{debug, error, info};

#[derive(Debug, Clone)]
pub struct StatsManager {
    db: Arc<Mutex<Connection>>,
}

impl StatsManager {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS unique_visitors (
                ip_address TEXT PRIMARY KEY,
                last_visit TEXT NOT NULL,
                visit_count INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS page_ranking (
                url TEXT PRIMARY KEY,
                visit_count INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS detailed_visits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ip_address TEXT NOT NULL,
                url TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS error_404 (
                url TEXT PRIMARY KEY,
                access_count INTEGER NOT NULL DEFAULT 0,
                last_access TEXT NOT NULL
            )",
            [],
        )?;

        info!("Statistics database initialized at {}", db_path);
        
        Ok(StatsManager {
            db: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn record_visit(&self, ip_address: &str, url: &str) {
        if let Err(e) = self.record_visit_internal(ip_address, url) {
            error!("Failed to record visit for {} on {}: {}", ip_address, url, e);
        }
    }

    pub fn record_404_error(&self, url: &str) {
        if let Err(e) = self.record_404_error_internal(url) {
            error!("Failed to record 404 error for {}: {}", url, e);
        }
    }

    fn record_visit_internal(&self, ip_address: &str, url: &str) -> Result<()> {
        let db = self.db.lock().map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_BUSY),
                Some(format!("Failed to acquire lock: {}", e))
            )
        })?;

        let now = Utc::now().to_rfc3339();

        // Update unique visitors
        db.execute(
            "INSERT INTO unique_visitors (ip_address, last_visit, visit_count) VALUES (?1, ?2, 1)
             ON CONFLICT(ip_address) DO UPDATE SET 
             last_visit = ?2, 
             visit_count = visit_count + 1",
            [ip_address, &now],
        )?;

        // Update page ranking
        db.execute(
            "INSERT INTO page_ranking (url, visit_count) VALUES (?1, 1)
             ON CONFLICT(url) DO UPDATE SET visit_count = visit_count + 1",
            [url],
        )?;

        // Insert detailed visit
        db.execute(
            "INSERT INTO detailed_visits (ip_address, url, timestamp) VALUES (?1, ?2, ?3)",
            [ip_address, url, &now],
        )?;

        debug!("Recorded visit: {} -> {}", ip_address, url);
        Ok(())
    }

    fn record_404_error_internal(&self, url: &str) -> Result<()> {
        let db = self.db.lock().map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_BUSY),
                Some(format!("Failed to acquire lock: {}", e))
            )
        })?;

        let now = Utc::now().to_rfc3339();

        // Update 404 errors tracking
        db.execute(
            "INSERT INTO error_404 (url, access_count, last_access) VALUES (?1, 1, ?2)
             ON CONFLICT(url) DO UPDATE SET 
             access_count = access_count + 1, 
             last_access = ?2",
            [url, &now],
        )?;

        debug!("Recorded 404 error: {}", url);
        Ok(())
    }

    pub fn get_page_stats(&self, url: &str) -> Option<i64> {
        if let Ok(db) = self.db.lock() {
            let mut stmt = db.prepare("SELECT visit_count FROM page_ranking WHERE url = ?1").ok()?;
            let result: Result<i64> = stmt.query_row([url], |row| row.get(0));
            result.ok()
        } else {
            None
        }
    }

    pub fn get_unique_visitor_count(&self) -> Option<i64> {
        if let Ok(db) = self.db.lock() {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM unique_visitors").ok()?;
            let result: Result<i64> = stmt.query_row([], |row| row.get(0));
            result.ok()
        } else {
            None
        }
    }
}