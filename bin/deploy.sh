#!/bin/bash
set -e

# Configuration
INSTALL_DIR="$HOME/.local/bin/odx"
BINARY_NAME="jynx"
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
DEPLOYABLE="${BINARY_NAME}"

echo "🎨 Building jynx..."
cd "$PROJECT_DIR"
if ! cargo build --release; then
    echo "❌ Build failed!"
    exit 1
fi

# Check if binary was created
if [ ! -f "target/release/${DEPLOYABLE}" ]; then
    echo "❌ Binary not found at target/release/${DEPLOYABLE}"
    exit 1
fi

echo "📦 Deploying to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

if ! cp "target/release/${DEPLOYABLE}" "$INSTALL_DIR/$BINARY_NAME"; then
    echo "❌ Failed to copy binary to $INSTALL_DIR"
    exit 1
fi

if ! chmod +x "$INSTALL_DIR/$BINARY_NAME"; then
    echo "❌ Failed to make binary executable"
    exit 1
fi

# Verify deployment
if [ ! -x "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo "❌ Binary is not executable at $INSTALL_DIR/$BINARY_NAME"
    exit 1
fi

# Install default themes to XDG+ location
echo "📂 Installing default themes to XDG+ location..."
THEME_DIR="$HOME/.local/etc/rsb/jynx/themes"
mkdir -p "$THEME_DIR"

# Copy default themes with proper naming convention
if [ -f "$PROJECT_DIR/themes/example-theme.yml" ]; then
    cp "$PROJECT_DIR/themes/example-theme.yml" "$THEME_DIR/theme_default.yml"
    echo "   ✅ Installed theme_default.yml"
fi

if [ -f "$PROJECT_DIR/themes/kb-default.yml" ]; then
    cp "$PROJECT_DIR/themes/kb-default.yml" "$THEME_DIR/theme_kb.yml"
    echo "   ✅ Installed theme_kb.yml"
fi

# Test the binary with jynx-specific features
echo "🧪 Testing binary..."
if ! echo "Test https://example.com version 1.0.0" | "$INSTALL_DIR/$BINARY_NAME" > /dev/null 2>&1; then
    echo "❌ Binary test failed!"
    exit 1
fi

echo "✅ Deployed successfully!"
echo ""
echo "📍 Binary location: $INSTALL_DIR/$BINARY_NAME"
echo ""
echo "💡 Usage examples:"
echo "   # Basic highlighting"
echo "   cat file.txt | \"$INSTALL_DIR/$BINARY_NAME\""
echo ""
echo "   # With XDG+ theme resolution"
echo "   cat logs.txt | \"$INSTALL_DIR/$BINARY_NAME\" --theme=default --filter logs"
echo ""
echo "   # Theme management"
echo "   \"$INSTALL_DIR/$BINARY_NAME\" theme list"
echo "   \"$INSTALL_DIR/$BINARY_NAME\" theme create my_theme"
echo ""
echo "   # Fixed width output"
echo "   echo \"Deploy v1.2.3\" | \"$INSTALL_DIR/$BINARY_NAME\" --width 80 --align center"
echo ""
echo "🧪 Quick test:"
echo "Test :critical: issue at https://example.com v1.2.3" | "$INSTALL_DIR/$BINARY_NAME"