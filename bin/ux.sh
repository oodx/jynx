#!/bin/bash
#
# jynx UX Demo - Show off jynx's intelligent highlighting capabilities
#

JYNX="./target/release/jynx"

# Build release version if it doesn't exist
if [ ! -f "$JYNX" ]; then
    echo "🔨 Building release version..."
    cargo build --release
fi

echo "🎨 jynx - Intelligent Syntax Highlighter Demo"
echo "============================================="
echo

# Auto-detection showcase
echo "🤖 Auto-Detection Intelligence:"
echo "-------------------------------"
echo "Check https://api.github.com/v1.2.3 for deployment at /prod/server.conf" | $JYNX
echo "Visit https://docs.rust-lang.org version 1.75.0 in ~/projects/rust/main.rs" | $JYNX
echo "Download v2.1.0-beta from https://releases.ubuntu.com to /opt/install.sh" | $JYNX
echo

# Icon mapping showcase
echo "🎯 Icon Mapping System (:word: patterns):"
echo "------------------------------------------"
echo "Status: :critical: issue found, :success: tests passing, :warning: memory low" | $JYNX
echo "Deploy: :rocket: launching v1.0.0 to :server: production :check: verified" | $JYNX
echo "Debug: :magic: somehow works, :silly: impossible value, :think: processing..." | $JYNX
echo

# Theme system with different filters
echo "🎨 Theme System with Filters:"
echo "-----------------------------"
echo "ERROR: Database connection failed, WARNING: High memory usage, INFO: Process started" | $JYNX --theme themes/example-theme.yml --filter logs
echo "URGENT: Fix critical bug, TODO: Add tests, DONE: Deploy complete" | $JYNX --theme themes/example-theme.yml --filter todo
echo "def process_data(): return results # Main processing function" | $JYNX --theme themes/example-theme.yml --filter code
echo

# Width and alignment formatting
echo "📏 Fixed Width Output Formatting:"
echo "---------------------------------"
echo "Deploy :success: complete v1.2.3" | $JYNX --width 40 --align left
echo "Deploy :success: complete v1.2.3" | $JYNX --width 40 --align center  
echo "Deploy :success: complete v1.2.3" | $JYNX --width 40 --align right
echo

# Complex combination showcase
echo "🚀 Complex Feature Combinations:"
echo "--------------------------------"
echo ":keeper: reviewed CRITICAL security issue at https://vuln-db.com/CVE-2024-1234 version 3.2.1 in /etc/config.json" | $JYNX --theme themes/example-theme.yml --filter troubleshoot --width 80 --align center
echo

# Real-world examples
echo "🌍 Real-World Usage Examples:"
echo "-----------------------------"

# Log processing
echo "# Processing server logs:"
echo "2024-01-15 14:30:00 INFO: Server started on https://app.example.com:8080" | $JYNX --theme themes/example-theme.yml --filter logs
echo "2024-01-15 14:30:15 ERROR: Database connection timeout to db.example.com:5432" | $JYNX --theme themes/example-theme.yml --filter logs
echo "2024-01-15 14:30:30 SUCCESS: Deploy v2.1.0 completed in /opt/app/" | $JYNX --theme themes/example-theme.yml --filter logs
echo

# Development workflow
echo "# Code review output:"
echo "function processUser(user@domain.com): validate input v1.0.0 # TODO: Add validation" | $JYNX --theme themes/example-theme.yml --filter code
echo "git commit a1b2c3d: Fix :critical: bug in /src/auth.rs https://github.com/user/repo" | $JYNX --theme themes/example-theme.yml --filter code
echo

# System monitoring
echo "# System status monitoring:"
echo "CPU: 85% :warning: high usage, Memory: 16GB :success: available, Disk: /var/log 90% :critical:" | $JYNX --theme themes/example-theme.yml --filter troubleshoot --width 90 --align center
echo

# Color template system showcase (%c:colorname(text))
echo "🎨 Color Template System (%c:colorname(text)):"
echo "-----------------------------------------------"
echo "Simple templates: %c:red(ERROR) %c:green(SUCCESS) %c:amber(WARNING) %c:blue(INFO)" | $JYNX
echo "Extended palette: %c:crimson(CRITICAL) %c:emerald(VERIFIED) %c:coral(ALERT) %c:azure(DEBUG)" | $JYNX
echo

# Template edge cases
echo "🔧 Template Edge Cases:"
echo "-----------------------"
echo "Empty parentheses: %c:red(())" | $JYNX
echo "Square brackets: %c:blue([value]) %c:purple([key=data])" | $JYNX
echo "Percent signs: %c:green(%test%) %c:yellow(progress: 85%)" | $JYNX
echo "Function calls: %c:amber(function(param)) %c:orange(validate(user@domain.com))" | $JYNX
echo "Mixed symbols: %c:crimson(Error: (code 42)) %c:teal({key: \"value\"})" | $JYNX
echo

# No-color mode demonstration
echo "🔇 No-Color Mode (--no-color):"
echo "------------------------------"
echo "With colors:"
echo "Status: %c:emerald(SUCCESS) - %c:crimson(3 errors) found in %c:blue(5 files)" | $JYNX
echo "Without colors (same input):"
echo "Status: %c:emerald(SUCCESS) - %c:crimson(3 errors) found in %c:blue(5 files)" | $JYNX --no-color
echo

# Advanced template patterns
echo "🎯 Advanced Template Patterns:"
echo "------------------------------"
echo "Build status: %c:lime(✓ PASSED) tests: %c:emerald(42/42) coverage: %c:gold(98%)" | $JYNX
echo "Deploy pipeline: %c:azure(→ staging) → %c:amber(→ testing) → %c:crimson(✗ FAILED)" | $JYNX
echo "System health: CPU %c:green(35%) RAM %c:yellow(67%) Disk %c:red(89%) Network %c:blue(12Mbps)" | $JYNX
echo

# Real-world templating examples
echo "🌍 Real-World Template Examples:"
echo "--------------------------------"
echo "Git status: %c:green(M) modified.rs %c:red(D) deleted.py %c:blue(?) untracked.md %c:purple(A) added.txt" | $JYNX
echo "Test results: %c:emerald(PASS) unit_tests %c:amber(SKIP) integration_tests %c:crimson(FAIL) performance_tests" | $JYNX
echo "Docker: %c:cyan(RUNNING) webapp:v1.2.3 %c:gray(STOPPED) redis:latest %c:green(HEALTHY) postgres:14" | $JYNX
echo "Security scan: %c:red(HIGH) 3 vulns %c:orange(MEDIUM) 7 vulns %c:yellow(LOW) 12 vulns %c:green(CLEAN) 0 secrets" | $JYNX
echo

# Template + theme combination
echo "🎨 Templates + Themes Combined:"
echo "-------------------------------"
echo "Log entry with templates: %c:steel([INFO]) Server started %c:success(OK) listening on %c:azure(http://localhost:8080)" | $JYNX --theme themes/example-theme.yml --filter logs
echo "Code review with templates: %c:violet([REVIEW]) Function %c:amber(getUserData()) needs %c:crimson([CRITICAL]) security fix" | $JYNX --theme themes/example-theme.yml --filter code
echo

# Extended semantic colors showcase
echo "🎭 Extended Semantic Colors:"
echo "----------------------------"
echo ":silly: This should never happen :magic: but somehow it works :trace: entering function :think: computing result" | $JYNX --theme themes/example-theme.yml --filter troubleshoot
echo

echo "✨ jynx makes your terminal output beautiful and informative!"
echo "🚰 Perfect for Unix pipelines: cat file.txt | jynx | less -R"
echo "📚 Full theme customization with YAML configuration files"
echo