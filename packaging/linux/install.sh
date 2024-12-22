#!/bin/bash

set -e

APP_NAME="Rhyolite"   # Replace with your app's name
BINARY_URL="https://objects.githubusercontent.com/github-production-release-asset-2e65be/904347772/9d70dabc-59a8-4855-b52a-94c689a5adce?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=releaseassetproduction%2F20241222%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20241222T191443Z&X-Amz-Expires=300&X-Amz-Signature=a26d0c5af840435604433cdc744fe0e9fee8c673618699d01eb758445ed43bcc&X-Amz-SignedHeaders=host&response-content-disposition=attachment%3B%20filename%3DRhyolite_0.1.1_linux_binary&response-content-type=application%2Foctet-stream"  # Update with your binary's URL
ICON_URL="blob:https://github.com/de607161-a14b-4f2c-87fe-0c06b2865000"  # Update with your app's icon URL
INSTALL_DIR="$HOME/.local/bin/Rhyolite"
DESKTOP_FILE_DIR="$HOME/.local/share/applications" # For a user-specific install
ICON_DIR="$HOME/.local/share/icons"

echo "Installing $APP_NAME..."

# Download the binary
curl -L $BINARY_URL -o /tmp/$APP_NAME
chmod +x /tmp/$APP_NAME

# Move the binary to the installation directory
mkdir -p $INSTALL_DIR/Rhyolite
mv /tmp/$APP_NAME $INSTALL_DIR/$APP_NAME

# Download and install the icon
mkdir -p $ICON_DIR
curl -L $ICON_URL -o /tmp/$APP_NAME.png
mv /tmp/$APP_NAME.png $ICON_DIR/$APP_NAME.png

# Create a .desktop file
mkdir -p $DESKTOP_FILE_DIR
DESKTOP_FILE=$DESKTOP_FILE_DIR/$APP_NAME.desktop

cat <<EOF > $DESKTOP_FILE
[Desktop Entry]
Type=Application
Name=$APP_NAME
Comment=A simple text editor written in Rust using Tauri.
Exec=$INSTALL_DIR/$APP_NAME
Icon=$ICON_DIR/$APP_NAME.png
Terminal=false
Categories=Utility;TextEditor;
Author=Suyog Tandel
EOF

chmod +x $DESKTOP_FILE

echo "$APP_NAME installed successfully!"

# Refresh desktop database
if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$HOME/.local/share/applications"
fi

echo "You can now find $APP_NAME in your application menu."
