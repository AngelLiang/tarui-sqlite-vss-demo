#!/bin/sh
ARCH=$(uname -m)

if [ "$ARCH" == "arm64" ]; then
    # 处理libomp.dylib
    if [ ! -f "src-tauri/libs/arm64/libomp.dylib" ]; then
        cp /opt/homebrew/opt/libomp/liblibomp.dylib src-tauri/libs/arm64/
        echo "拷贝libomp.dylib"
        install_name_tool -id @executable_path/../Frameworks/libomp.dylib src-tauri/libs/arm64/libomp.dylib
        echo "修改libomp.dylib的链接路径"
    fi

    LIB_SQLITE_VSS=$PWD/src-tauri/libs/arm64/sqlite-vss-v0.1.2-static-macos-aarch64 yarn tauri dev --verbose
elif [ "$ARCH" == "x86_64" ]; then
    # 处理libomp.dylib
    if [ ! -f "src-tauri/libs/x86_64/libomp.dylib" ]; then
        cp /usr/local/opt/llvm/lib/libomp.dylib src-tauri/libs/x86_64/
        echo "拷贝libomp.dylib"
        install_name_tool -id @executable_path/../Frameworks/libomp.dylib src-tauri/libs/x86_64/libomp.dylib
        echo "修改libomp.dylib的链接路径"
    fi

    LIB_SQLITE_VSS=$PWD/src-tauri/libs/x86_64/sqlite-vss-v0.1.2-static-macos-x86_64 yarn tauri dev --verbose
else
    echo "Unknown architecture: $ARCH"
fi
