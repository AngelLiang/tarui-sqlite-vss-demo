ARCH=$(uname -m)

if [ "$ARCH" == "arm64" ]; then
    LIB_SQLITE_VSS=$PWD/src-tauri/libs/sqlite-vss-v0.1.2-static-macos-aarch64 yarn tauri build --verbose
elif [ "$ARCH" == "x86_64" ]; then
    LIB_SQLITE_VSS=$PWD/src-tauri/libs/sqlite-vss-v0.1.2-static-macos-x86_64 yarn tauri build --verbose
else
    echo "Unknown architecture: $ARCH"
fi
