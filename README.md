# tauri-sqlite-vss-demo

tauri-app使用sqlite-vss示例

## 开发环境和依赖

- macOS
- yarn
- tauri
- rusqlite v0.29.0
- sqlite-vss v0.1.2

## 准备工作

```
brew install libomp
brew install llvm
```

## 运行

把 LIB_SQLITE_VSS 变量换为 lib/sqlite-vss-v0.1.2-static-macos-x86_64 的绝对路径，再运行`yarn tauir dev`命令：

```
LIB_SQLITE_VSS=/path/to/tauri-sqlite-vss-demo/lib/sqlite-vss-v0.1.2-static-macos-x86_64 yarn tauri dev
```

## 需要注意的地方

1. cargo.toml中的sqlite-vss依赖如果设置如下`sqlite-vss = {version="0.1.2"}`，就需要添加LIB_SQLITE_VSS环境变量指示sqlite-vss-v0.1.2-static-macos-x86_64库的位置；
2. src-tauir/build.rs需要添加针对sqlite-vss的编译语句；
3. rusqlite暂时只能使用0.29.0版本，0.30.0以上的版本可能会报错；
