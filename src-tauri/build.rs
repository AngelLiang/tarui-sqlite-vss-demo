fn main() {
    // 编译sqlite-vss需要添加下面两条语句
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup,-lomp");
        println!("cargo:rustc-flags=-L/usr/local/opt/llvm/lib -lblas -llapack");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup,-lstdc++");
    }
    tauri_build::build()
}
