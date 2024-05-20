use std::env;

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


fn build_simple() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut cfg = cc::Build::new();

    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // 获取Cargo.toml所在目录的路径

    cfg.include(format!("{}/../simple/src", manifest_dir));
    cfg.file(format!("{}/../simple/src/pinyin.h", manifest_dir));
    cfg.file(format!("{}/../simple/src/simple_highlight.h", manifest_dir));
    cfg.file(format!("{}/../simple/src/simple_tokenizer.h", manifest_dir));
    cfg.file(format!("{}/../simple/src/pinyin.cc", manifest_dir));
    cfg.file(format!("{}/../simple/src/simple_highlight.cc", manifest_dir));
    cfg.file(format!("{}/../simple/src/simple_tokenizer.cc", manifest_dir));
    cfg.file(format!("{}/../simple/src/entry.cc", manifest_dir));

    cfg.include(format!("{}/../simple/contrib/sqlite3", manifest_dir));

    cfg.include(format!("{}/../cmrc/include", manifest_dir));
    cfg.file(format!("{}/../cmrc/pinyin.txt/lib.cpp", manifest_dir));
    cfg.file(format!("{}/../cmrc/pinyin.txt/pinyin.txt.cpp", manifest_dir));

    if cfg!(feature = "jieba") {
        cfg
            .define("USE_JIEBA", "1")
            .include(format!("{}/../cppjieba/include", manifest_dir))
            .include(format!("{}/../cppjieba/deps/limonp/include", manifest_dir));
    }

    cfg
        .cpp(true)
        .std("c++14")
        .flag_if_supported("/utf-8");
    cfg.compile("simple");
    println!("cargo:lib_dir={out_dir}");
}


fn main() {
    build_simple();
    build();
    tauri_build::build()
}
