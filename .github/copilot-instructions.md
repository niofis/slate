# Slate CMS

Slate is a custom Content Management System built in Rust using the actix-web framework. It serves static content (HTML, Markdown, CSS, JavaScript, images, fonts) from a file-based structure where folders represent URL paths.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Bootstrap, Build, and Test the Repository
- Install Rust toolchain if not available: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- `cargo build` -- takes ~60 seconds. NEVER CANCEL. Set timeout to 90+ minutes.
- `cargo build --release` -- takes ~76 seconds. NEVER CANCEL. Set timeout to 90+ minutes.
- `cargo test` -- takes <2 seconds. No tests exist, so this validates project compiles.

### Run the Application
- ALWAYS run the build steps first.
- `cargo run` -- starts development server on 127.0.0.1:8080
- Server starts in <1 second and logs "Server started on 127.0.0.1:8080"
- Use Ctrl+C to stop the server

### Code Quality and Linting
- `cargo fmt --check` -- check formatting (<1 second)
- `cargo fmt` -- apply formatting fixes
- `cargo clippy` -- run linter (~22 seconds). NEVER CANCEL. Set timeout to 30+ minutes.
- Always run `cargo fmt` and `cargo clippy` before committing changes

## Content Structure and Validation

### Content Directory Structure
The application serves content from `./pages/` directory:
- `pages/content.html` or `pages/content.md` -- root page content
- `pages/template.html` -- optional template file for markdown rendering
- `pages/config.json` -- optional configuration for headers
- `pages/subfolder/content.html` -- content for `/subfolder/` route
- `pages/assets/style.css` -- static assets served directly

### Manual Validation (CRITICAL)
Since there are no automated tests, ALWAYS manually validate changes:

1. **Create test content structure:**
   ```bash
   mkdir -p pages/blog pages/assets
   echo "<h1>Home Page</h1>" > pages/content.html
   echo "# Blog Post" > pages/blog/content.md
   echo "body { color: blue; }" > pages/assets/style.css
   ```

2. **Create template for markdown (optional):**
   ```bash
   cat > pages/template.html << 'EOF'
   <!DOCTYPE html>
   <html>
   <head><title>My Site</title></head>
   <body>
       <header>My CMS</header>
       <main><!-- content --></main>
       <footer>© 2024</footer>
   </body>
   </html>
   EOF
   ```

3. **Start server and test endpoints:**
   ```bash
   cargo run &
   sleep 2
   curl -s http://127.0.0.1:8080/          # Should return HTML content
   curl -s http://127.0.0.1:8080/blog/     # Should return rendered markdown
   curl -s http://127.0.0.1:8080/assets/style.css  # Should return CSS
   kill %1  # Stop background server
   ```

4. **ALWAYS test these scenarios after making changes:**
   - HTML file serving
   - Markdown to HTML conversion
   - Template rendering with markdown
   - Static asset serving (CSS, JS)
   - URL path mapping (folders to routes)

## Architecture Overview

### Key Components
- `src/main.rs` -- Application entry point, sets up communication channels
- `src/server.rs` -- HTTP server using actix-web, handles requests
- `src/content_manager.rs` -- Background thread that processes content requests
- `src/file_system.rs` -- File operations, reads from `./pages/` directory
- `src/web_content.rs` -- Content processing, markdown rendering
- `src/templating.rs` -- Template system using HTML comment slots
- `src/types.rs` -- Type definitions and data structures

### Content Types Supported
- HTML (`.html`) -- served directly
- Markdown (`.md`) -- converted to HTML, optionally templated
- CSS (`.css`) -- served with text/css content-type
- JavaScript (`.js`) -- served with application/javascript content-type
- Images: JPEG, PNG, ICO, SVG
- Fonts: WOFF2
- WebAssembly (`.wasm`)

### Template System
- Templates use HTML comment slots: `<!-- slotname -->`
- Markdown files are rendered and inserted into `<!-- content -->` slot
- Template filename in file hierarchy determines which template is used

## Common Development Tasks

### Building and Testing Changes
```bash
# Full development cycle
cargo fmt
cargo clippy  # ~22 seconds, NEVER CANCEL
cargo build   # ~60 seconds, NEVER CANCEL
cargo run &   # Start server
# Manual validation steps here
kill %1       # Stop server
```

### Adding New Content Types
1. Add new variant to `WebContent` enum in `src/types.rs`
2. Add content-type mapping in `src/server.rs` index function
3. Add file extension handling in `src/web_content.rs` process_content function
4. Test by creating file and verifying server response

### Debugging Content Issues
- Check file exists in `./pages/` directory
- Verify URL path matches folder structure (trailing slash required for folders)
- Test direct file access vs. folder content.html/content.md access
- Validate template.html syntax if using templating

## Common Command Outputs

### Repository Structure
```
.
├── Cargo.toml          # Rust project configuration
├── Cargo.lock          # Dependency lock file
├── README.md           # Basic project info
├── src/                # Source code
│   ├── main.rs
│   ├── server.rs
│   ├── content_manager.rs
│   ├── file_system.rs
│   ├── web_content.rs
│   ├── templating.rs
│   └── types.rs
└── pages/              # Content directory (created at runtime)
    ├── content.html
    ├── template.html
    └── subfolder/
        └── content.md
```

### Dependencies (Cargo.toml)
```toml
[dependencies]
actix-web = "4"
markdown = "1.0.0-alpha.23"
notify = "8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1"
```

## Timing Expectations
- **NEVER CANCEL builds or linting commands**
- `cargo build`: ~60 seconds (set timeout to 90+ minutes)
- `cargo build --release`: ~76 seconds (set timeout to 90+ minutes)
- `cargo clippy`: ~22 seconds (set timeout to 30+ minutes)
- `cargo test`: <2 seconds
- `cargo fmt`: <1 second
- `cargo run`: <1 second to start server

## Known Issues and Warnings
- Build produces 3 dead code warnings - these are expected and safe to ignore
- No automated tests exist - manual validation is required
- Server requires `pages/` directory structure to serve content properly
- Trailing slash is enforced for directory URLs (automatic redirect)