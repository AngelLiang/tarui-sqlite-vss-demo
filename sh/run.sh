ARCH=$(uname -m)

if [ "$ARCH" == "arm64" ]; then
    LIB_SQLITE_VSS=$PWD/src-tauri/libs/arm64/sqlite-vss-v0.1.2-static-macos-aarch64 yarn tauri dev --verbose
elif [ "$ARCH" == "x86_64" ]; then
    LIB_SQLITE_VSS=$PWD/src-tauri/libs/x86_64/sqlite-vss-v0.1.2-static-macos-x86_64 yarn tauri dev --verbose
else
    echo "Unknown architecture: $ARCH"
fi
