# RSB v2.0 Quick Reference Guide

## Command Execution
```rust
// Execute and get output (exits on failure)
run!("ls -la")                          // Returns String output  
run!("grep pattern file.txt", silent)   // Returns String (empty on failure)

// Execute and get full result (allows error handling)
let result = shell!("ls /nonexistent"); // Returns CmdResult
if result.status != 0 {
    // Handle error: result.error contains stderr
}
```

## System Information
```rust
hostname!()         // Get system hostname
user!()             // Get current username  
home_dir!()         // Get user's home directory
current_dir!()      // Get current working directory
```

## Network Operations  
```rust
get!("https://api.com/data")                           // Simple HTTP GET
curl!("https://api.com", options: "-H 'Auth: token'") // GET with options  
curl!(post: "https://api.com", data: "payload")       // HTTP POST
```

## Process Management
```rust
pid_of!("nginx")                    // Get PID of process
process_exists!("my-daemon")        // Check if process is running
kill_pid!("1234")                   // Kill process by PID (TERM)
kill_pid!("1234", signal: "KILL")  // Kill with specific signal
kill_process!("nginx")              // Kill all processes by name
```

## File Locking
```rust
with_lock!("/tmp/script.lock" => {  // Exclusive execution block
    // Critical operations here
});

lock!("/tmp/script.lock");          // Manual lock acquisition
unlock!("/tmp/script.lock");        // Manual lock release  
```

## Archive Operations
```rust
pack!("backup.tar.gz", "dir1/", "file.txt")  // Auto-detect format
tar!(create: "backup.tar", "dir1/", "file.txt")     // Create tar
tar!(extract: "backup.tar")                          // Extract here
tar!(extract: "backup.tar", to: "dest/")            // Extract to directory
tar!(list: "backup.tar")                            // List contents

zip!(create: "backup.zip", "dir1/", "file.txt")     // Create zip
unpack!("any-archive.tar.gz")                       // Auto-detect extract
```

## Advanced Stream Processing  
```rust
// Line range extraction
sed_lines!(content, 10, 20)        // Lines 10-20 (inclusive)

// Context around matches  
sed_around!(content, "ERROR", 3)   // 3 lines before/after ERROR

// Template insertion (unique sentinel)
sed_insert!(new_content, "{{PLACEHOLDER}}", source)

// Template replacement (all occurrences) 
sed_template!(replacement, "{{VAR}}", source)

// Simple string replacement
sed_replace!(source, "old", "new")

// File-based versions
sed_lines_file!("file.txt", 1, 5)
sed_template_file!("template.html", "Alice", "{{NAME}}")
```

## JSON Processing (requires jq)
```rust
json_get!(json_string, ".user.name")       // Extract from JSON string
json_get_file!("config.json", ".api.key")  // Extract from JSON file
```

## Combined Workflow Examples

### System-Aware Deployment
```rust  
fn deploy_app(args: Args) -> i32 {
    let host = hostname!();
    let user = user!();
    
    info!("Deploying to {} as {}", host, user);
    
    with_lock!("/tmp/deploy.lock" => {
        // Download deployment package
        let package_url = format!("https://releases.com/app-{}.tar.gz", get_var("VERSION"));
        curl!(&package_url, options: "-o app.tar.gz");
        
        // Stop existing service  
        if process_exists!("app-server") {
            kill_process!("app-server");
        }
        
        // Deploy new version
        unpack!("app.tar.gz", to: "/opt/app");
        
        // Start service in background
        job!(background: "systemctl start app-server");
        
        okay!("Deployment completed on {}", host);
    });
    
    0
}
```

### Log Processing Pipeline
```rust
fn analyze_logs(args: Args) -> i32 {
    let log_file = args.get_or(1, "app.log");
    
    // Extract errors with context
    let errors = sed_around_file!(&log_file, "ERROR", 2);
    
    // Process through pipeline
    let error_summary = pipe!(&errors)
        .sed_template(&hostname!(), "{{HOST}}")
        .sed_template(&date!(human), "{{DATE}}")  
        .uniq()
        .to_string();
        
    // Generate report
    let report_file = format!("error-report-{}.txt", date!(epoch));
    write_file(&report_file, &error_summary);
    
    // Archive results
    pack!("log-analysis.tar.gz", &report_file);
    
    okay!("Log analysis complete");  
    0
}
```

### API Integration Workflow
```rust
fn sync_with_api(args: Args) -> i32 {
    // Load configuration
    let api_key = json_get_file!("config.json", ".credentials.api_key");
    let base_url = json_get_file!("config.json", ".api.base_url");
    
    if api_key.is_empty() || base_url.is_empty() {
        error!("Missing API configuration");
        return 1;
    }
    
    // Prepare request
    let auth_header = format!("Authorization: Bearer {}", api_key);
    let sync_url = format!("{}/sync/status", base_url);
    
    // Check sync status  
    let response = curl!(&sync_url, options: &format!("-H '{}'", auth_header));
    let needs_sync = json_get!(&response, ".sync_required");
    
    if needs_sync == "true" {
        info!("Synchronization required");
        
        // Create data package
        pack!("sync-data.tar.gz", "data/", "config/");
        
        // Upload data
        let upload_url = format!("{}/upload", base_url);  
        curl!(post: &upload_url, data: "@sync-data.tar.gz");
        
        okay!("Sync completed");
    } else {
        info!("No sync needed");  
    }
    
    0
}
```

## Tool Availability Checks
```rust
// Always check before using external tools
if !is_command("jq") {
    warn!("jq not available, JSON processing disabled");
}

if !is_command("curl") {
    error!("curl is required for network operations");
    return 1;
}

// Graceful degradation
let data = if is_command("jq") {
    json_get_file!("config.json", ".advanced_config")  
} else {
    // Fallback to simple parsing
    cat!("config.json").grep("simple_setting").cut(2, ":").trim()
};
```

## Error Handling Patterns
```rust  
// High-order functions validate user input
fn do_backup(args: Args) -> i32 {
    require_var!("BACKUP_DIR");
    require_command!("tar");
    
    let source = args.get_or(1, ".");
    let dest = get_var("BACKUP_DIR");
    
    validate!(is_dir(&source), "Source directory not found: {}", source);
    
    _create_backup(&source, &dest)
}

// Mid-level functions handle business logic
fn _create_backup(source: &str, dest: &str) -> i32 {
    let backup_name = format!("backup-{}-{}.tar.gz", hostname!(), date!(epoch));
    let backup_path = format!("{}/{}", dest, backup_name);
    
    if __archive_directory(source, &backup_path) {
        okay!("Backup created: {}", backup_name);
        0
    } else {
        error!("Backup failed");
        1
    }
}

// Low-level functions trust inputs and handle system errors
fn __archive_directory(source: &str, dest: &str) -> bool {
    match create_tar_gz(dest, &[source]) {
        result if result.status == 0 => true,
        result => {
            error!("Archive creation failed: {}", result.error);
            false
        }
    }
}
```

RSB v2.0 provides a complete toolkit for automation while maintaining string-first simplicity and bash-like ergonomics.