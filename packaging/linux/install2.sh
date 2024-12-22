#!/usr/bin/env sh
set -eu

# Modify these variables as needed
APP_ICON_URL="https://github.com/RedddFoxxyy/Rhyolite/blob/master/src-tauri/icons/icon.png"
APP_BINARY_URL="https://github.com/RedddFoxxyy/Rhyolite/releases/download/v0.1.1/Rhyolite_0.1.1_linux_binary"
APP_NAME="Rhyolite"
APP_ID="com.rhyolite.app"
APP_DESCRIPTION="A simple text editor written in Rust using Tauri."
APP_EXEC="./Rhyolite"
DESKTOP_FILE_PATH="$HOME/.local/share/applications/${APP_ID}.desktop"

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${APP_CHANNEL:-stable}"
    temp="$(mktemp -d "/tmp/${APP_NAME}-XXXXXX")"

    if [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    case "$platform-$arch" in
        linux-arm64* | linux-armhf | linux-aarch64)
            arch="aarch64"
            ;;
        linux-x86* | linux-i686*)
            arch="x86_64"
            ;;
        *)
            echo "Unsupported platform or architecture"
            exit 1
            ;;
    esac

    if command -v curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif command -v wget >/dev/null 2>&1; then
        curl () {
            wget -O- "$@"
        }
    else
        echo "Could not find 'curl' or 'wget' in your path"
        exit 1
    fi

    linux "$@"

    if [ -f "$HOME/.local/bin/$APP_NAME" ]; then
        echo "$APP_NAME has been installed. Run with '$APP_NAME'"
    else
        echo "To run $APP_NAME from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run $APP_NAME now, '~/.local/bin/$APP_NAME'"
    fi
}

linux() {
    if [ -n "${APP_BUNDLE_PATH:-}" ]; then
        cp "$APP_BUNDLE_PATH" "$temp/${APP_NAME}-linux-$arch"
        chmod +x "$temp/${APP_NAME}-linux-$arch"
    else
        echo "Downloading $APP_NAME"
        curl -o "$temp/${APP_NAME}-linux-$arch" "$APP_BINARY_URL"
        chmod +x "$temp/${APP_NAME}-linux-$arch"
    fi

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    # Unpack
    rm -rf "$HOME/.local/${APP_NAME}$suffix.app"
    mkdir -p "$HOME/.local/${APP_NAME}$suffix.app"
    mv "$temp/${APP_NAME}-linux-$arch" "$HOME/.local/${APP_NAME}$suffix.app/bin/"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications" "$HOME/.local/${APP_NAME}$suffix.app/share/icons/hicolor/512x512/apps"

    # Download the icon
    echo "Downloading icon"
    curl "$APP_ICON_URL" > "$HOME/.local/${APP_NAME}$suffix.app/share/icons/hicolor/512x512/apps/${APP_NAME}.png"

    # Write and modify the .desktop file
    echo "Creating .desktop file"
    cat > "$DESKTOP_FILE_PATH" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=$APP_NAME
Comment=$APP_DESCRIPTION
Exec=$HOME/.local/${APP_NAME}$suffix.app/bin/$APP_EXEC
Icon=$HOME/.local/${APP_NAME}$suffix.app/share/icons/hicolor/512x512/apps/${APP_NAME}.png
Terminal=false
Categories=Utility;TextEditor;
Author=Suyog Tandel
EOF

    echo "$APP_NAME installed successfully!"
}

main "$@"
