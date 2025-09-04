# jynx Support Scripts

This directory contains support scripts for building, deploying, and demonstrating jynx.

## Scripts

### `deploy.sh`
Production deployment script that:
- Builds jynx in release mode
- Installs to `~/.local/bin/odx/jynx`  
- Runs basic functionality tests
- Provides usage examples

```bash
./bin/deploy.sh
```

### `ux.sh` 
Interactive UX demonstration showcasing jynx's capabilities:
- Auto-detection intelligence (URLs, versions, paths)
- Icon mapping system (`:word:` patterns)
- Theme system with different filters
- Width/alignment formatting
- Real-world usage examples

```bash
./bin/ux.sh
```

## Usage

All scripts should be run from the project root directory:

```bash
# Deploy to production
./bin/deploy.sh

# See jynx in action
./bin/ux.sh
```

The UX demo is perfect for showing off jynx's intelligent highlighting to others!