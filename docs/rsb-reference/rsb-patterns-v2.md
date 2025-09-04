# RSB Design Patterns v2.0 - Complete Architecture Guide

## Overview

RSB (Rebel String-Based) has evolved into a complete framework for building automation tools. This document outlines the updated architectural patterns and best practices for leveraging RSB's full capabilities.

## Core Philosophy Reinforced

RSB maintains its core principles while expanding capabilities:

1. **String-First Design** - Everything is a string until proven otherwise
2. **Bash-Like Ergonomics** - Familiar patterns for shell script developers  
3. **Fail-Fast Simplicity** - Clear errors, immediate exit
4. **Zero-Cost Shell Integration** - Leverage existing system tools
5. **BashFX Function Ordinality** - Clear responsibility hierarchy

## Updated Architecture Layers

### Layer 1: System Interface
**New capabilities**: Extended system integration without dependencies

```rust
// System Information
let host = hostname!();
let user = user!(); 
let home = home_dir!();

// Network Operations  
let response = get!("https://api.service.com/data");
let result = curl!(post: "https://api.com", data: "payload");

// Process Management
let pid = pid_of!("nginx");
kill_process!("old-service");
```

**Pattern**: Use system interface layer for external integrations while maintaining string-first simplicity.

### Layer 2: Data Processing
**Enhanced capabilities**: Advanced text processing and templating

```rust
// Advanced Stream Processing
let results = cat!("large-log.txt")
    .sed_around("ERROR", 3)           // Context around errors
    .sed_template("$HOST", hostname!()) // Template replacement
    .to_string();

// Archive Operations
pack!("backup.tar.gz", "config/", "data/", "logs/");
unpack!("deployment.tar.gz", to: "/opt/app/");

// JSON Processing (when jq available)
let api_key = json_get_file!("config.json", ".api.credentials.key");
```

**Pattern**: Combine multiple processing operations in fluent chains while maintaining readability.

### Layer 3: Concurrency & State Management  
**New capabilities**: Robust process coordination and resource management

```rust
// Resource Locking
with_lock!("/tmp/deployment.lock" => {
    // Atomic deployment operations
    unpack!("new-version.tar.gz", to: "/opt/app/");
    kill_process!("app-server");
    job!(background: "systemctl start app-server");
});

// Advanced Job Management  
let build_job = job!(background: "cargo build --release");
let test_job = job!(background: "cargo test");

job!(wait: build_job);
job!(wait: test_job);
```

**Pattern**: Use locking for exclusive operations and job control for parallel processing.

## Updated Function Ordinality Patterns

### High-Order: User Interface Functions
**Enhanced with new system capabilities**

```rust
fn do_deploy(mut args: Args) -> i32 {
    // User-level validation (High-Order responsibility)
    require_var!("DEPLOY_TARGET");
    require_command!("docker");
    
    let target = args.get_or(1, &get_var("DEPLOY_TARGET"));
    let force = args.has_pop("--force");
    
    // System context awareness
    info!("Deploying to {} as user {}", target, user!());
    
    // Resource coordination
    with_lock!("/tmp/deploy.lock" => {
        if _deploy_to_target(&target, force) {
            okay!("Deployment successful to {}", target);
            0
        } else {
            error!("Deployment failed");
            1
        }
    })
}
```

### Mid-Level: Business Logic Functions  
**Leveraging new data processing capabilities**

```rust
fn _deploy_to_target(target: &str, force: bool) -> bool {
    // Environment-specific configuration
    let config = json_get_file!("deploy-config.json", &format!(".environments.{}", target));
    if config.is_empty() {
        error!("No configuration found for target: {}", target);
        return false;
    }
    
    // Advanced templating
    let deploy_script = cat!("deploy-template.sh")
        .sed_template(&get_var("VERSION"), "{{VERSION}}")
        .sed_template(target, "{{TARGET}}")
        .sed_template(&hostname!(), "{{DEPLOY_HOST}}")
        .to_string();
        
    write_file("deploy-generated.sh", &deploy_script);
    
    // Process coordination
    let result = __execute_deployment_script("deploy-generated.sh");
    result.status == 0
}
```

### Low-Level: System Operations
**Extended system integration**

```rust  
fn __execute_deployment_script(script_path: &str) -> CmdResult {
    // Pre-deployment health check
    if process_exists!("app-server") {
        info!("Stopping existing app-server process");
        kill_process!("app-server");
        sleep!(2); // Grace period
    }
    
    // Execute deployment with error handling
    let result = shell!("bash {}", script_path);
    
    // Post-deployment verification
    if result.status == 0 {
        // Wait for service to start
        for _ in 0..30 {
            if process_exists!("app-server") {
                break;
            }
            sleep!(1);
        }
    }
    
    result
}
```

## New Architectural Patterns

### Pattern 1: System-Aware Automation
**Use Case**: Scripts that adapt to their environment

```rust
fn setup_development_environment(args: Args) -> i32 {
    let host = hostname!();
    let user = user!();
    let home = home_dir!();
    
    info!("Setting up development environment for {} on {}", user, host);
    
    // Adaptive configuration based on system
    let config_template = match host.as_str() {
        h if h.starts_with("dev-") => "templates/dev-workstation.conf",
        h if h.starts_with("ci-") => "templates/ci-runner.conf", 
        _ => "templates/generic.conf"
    };
    
    cat!(config_template)
        .sed_template(&user, "{{USER}}")
        .sed_template(&home, "{{HOME}}")  
        .sed_template(&host, "{{HOSTNAME}}")
        .to_file(&format!("{}/.myapp-config", home));
        
    okay!("Environment configured for {}", host);
    0
}
```

### Pattern 2: API-Integrated Workflows  
**Use Case**: Scripts that interact with web services

```rust
fn sync_with_remote_service(args: Args) -> i32 {
    let api_key = json_get_file!("config.json", ".api_key");
    if api_key.is_empty() {
        error!("API key not found in config.json");
        return 1;
    }
    
    // Fetch remote data
    let options = format!("-H 'Authorization: Bearer {}'", api_key);
    let response = get!("https://api.service.com/sync/status", options: &options);
    
    // Process response
    let sync_needed = json_get!(&response, ".sync_required");
    if sync_needed == "true" {
        info!("Sync required, starting synchronization...");
        
        // Create sync package
        pack!("sync-data.tar.gz", "data/", "config/");
        
        // Upload (using curl POST)
        let upload_result = curl!(post: "https://api.service.com/upload", 
                                 data: "@sync-data.tar.gz");
        
        if !upload_result.is_empty() {
            okay!("Sync completed successfully");
            0  
        } else {
            error!("Sync upload failed");
            1
        }
    } else {
        info!("No sync required");
        0
    }
}
```

### Pattern 3: Concurrent Processing with Coordination
**Use Case**: Parallel operations with synchronization

```rust
fn parallel_build_and_test(args: Args) -> i32 {
    info!("Starting parallel build and test pipeline");
    
    with_lock!("/tmp/build-pipeline.lock" => {
        // Start parallel jobs
        let build_job = job!(background: "cargo build --release");
        let lint_job = job!(background: "cargo clippy");
        let doc_job = job!(background: "cargo doc");
        
        // Process other tasks while jobs run
        info!("Running jobs: build={}, lint={}, doc={}", build_job, lint_job, doc_job);
        
        // Wait for critical path (build)
        let build_status = job!(wait: build_job);
        if build_status != 0 {
            error!("Build failed, aborting pipeline");
            return build_status;
        }
        
        // Start tests after build completes
        let test_job = job!(background: "cargo test");
        
        // Wait for all jobs
        let lint_status = job!(wait: lint_job);
        let doc_status = job!(wait: doc_job);  
        let test_status = job!(wait: test_job);
        
        // Report results
        if lint_status == 0 && doc_status == 0 && test_status == 0 {
            okay!("All pipeline stages completed successfully");
            0
        } else {
            error!("Pipeline failed - lint:{} doc:{} test:{}", 
                   lint_status, doc_status, test_status);
            1
        }
    })
}
```

### Pattern 4: Advanced Text Processing Workflows
**Use Case**: Complex document/log processing

```rust
fn process_application_logs(args: Args) -> i32 {
    let log_dir = args.get_or(1, "/var/log/myapp");
    let output_dir = args.get_or(2, "./reports");
    
    mkdir_p(&output_dir);
    
    // Process each log file
    file_in!(log_file in &log_dir => {
        if !log_file.ends_with(".log") {
            continue;
        }
        
        info!("Processing log file: {}", log_file);
        
        // Extract errors with context
        let error_report = cat!(&log_file)
            .sed_around("ERROR", 2)
            .sed_template(&hostname!(), "{{HOSTNAME}}")  
            .sed_template(&date!(human), "{{REPORT_DATE}}")
            .to_string();
            
        // Extract performance metrics
        let perf_metrics = cat!(&log_file)
            .grep("PERF:")
            .sed("s/.*PERF: //")
            .sort()
            .uniq()
            .to_string();
            
        // Generate reports
        let base_name = path_split!(&log_file, into: "LOG");
        let error_file = format!("{}/errors-{}.txt", output_dir, get_var("LOG_basename"));  
        let perf_file = format!("{}/performance-{}.txt", output_dir, get_var("LOG_basename"));
        
        write_file(&error_file, &error_report);
        write_file(&perf_file, &perf_metrics);
    });
    
    // Create summary archive
    pack!("log-analysis.tar.gz", &output_dir);
    okay!("Log analysis complete, results archived");
    0
}
```

## Updated Best Practices

### 1. System Integration
- **Always check tool availability**: Use `is_command()` before calling external tools
- **Provide graceful fallbacks**: Warn when optional tools aren't available
- **Use system context**: Leverage `hostname!()`, `user!()`, `home_dir!()` for adaptive behavior

### 2. Resource Management
- **Use locking for exclusive operations**: Prevent concurrent modification
- **Coordinate with job control**: Balance parallelism with resource constraints
- **Clean up properly**: Always ensure locks are released and temporary files cleaned

### 3. Data Processing
- **Chain operations fluently**: Use stream processing for complex transformations  
- **Template liberally**: Use `sed_template!()` for dynamic content generation
- **Archive thoughtfully**: Use `pack!()`/`unpack!()` for data transfer and backup

### 4. Error Handling Evolution
- **Layer-appropriate error handling**: Different strategies for each function level
- **Context-aware messages**: Include system information in error reports
- **Graceful degradation**: Continue operation when optional features fail

## Integration Points

### RSB + Traditional Rust
RSB now handles more use cases, reducing the need to drop into traditional Rust:

```rust
fn hybrid_application(args: Args) -> i32 {
    // RSB handles all the "plumbing"
    let config = json_get_file!("config.json", ".database");
    let host_info = format!("{}@{}", user!(), hostname!());
    
    // Traditional Rust for complex algorithms (when needed)
    let processed_data = if args.has("--complex-analysis") {
        heavy_computation_module::analyze(&config)
    } else {
        // Simple RSB processing suffices
        cat!("input-data.txt").grep("important").to_string()  
    };
    
    // RSB for output and coordination
    write_file("results.txt", &processed_data);
    info!("Analysis completed on {}", host_info);
    0
}
```

### RSB + External Services
Network capabilities enable service integration:

```rust
fn deploy_with_notifications(args: Args) -> i32 {
    let slack_webhook = get_var("SLACK_WEBHOOK");
    
    // Notify start
    if !slack_webhook.is_empty() {
        curl!(post: &slack_webhook, 
              data: r#"{"text":"🚀 Deployment starting on HOST"}"#
                   .replace("HOST", &hostname!()));
    }
    
    let result = _execute_deployment(args);
    
    // Notify completion
    if !slack_webhook.is_empty() {
        let status = if result == 0 { "✅ SUCCESS" } else { "❌ FAILED" };
        let message = format!(r#"{{"text":"{} Deployment completed on {}"}}"#, 
                             status, hostname!());
        curl!(post: &slack_webhook, data: &message);
    }
    
    result
}
```

## Conclusion

RSB v2.0 represents a mature, complete framework for automation scripting. The additions maintain the core string-first philosophy while dramatically expanding capabilities:

- **System integration** without complex dependencies
- **Network operations** for modern API-driven workflows  
- **Advanced concurrency** with proper resource management
- **Sophisticated text processing** for complex data workflows
- **JSON integration** for configuration and API interactions

RSB now truly covers the entire "too big for bash, too small for full Rust" spectrum while maintaining its accessibility and simplicity principles.