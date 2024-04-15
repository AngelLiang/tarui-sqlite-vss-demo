
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn build() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // 获取Cargo.toml所在目录的路径
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup,-lomp");
    // println!("cargo:rustc-flags=-L/usr/local/opt/llvm/lib -lblas -llapack");
    println!("cargo:rustc-link-search=native={}/libs/x86_64", manifest_dir);
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn build() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // 获取Cargo.toml所在目录的路径
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup,-lomp");
    println!("cargo:rustc-link-search=native={}/libs/arm64", manifest_dir);
}

fn main() {
    build();
    tauri_build::build()
}
