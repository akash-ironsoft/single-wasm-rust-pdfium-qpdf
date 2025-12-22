fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=assets/libpdfium.a");
    println!("cargo:rerun-if-changed=assets/libqpdf.a");

    // This project only targets wasm32-unknown-emscripten
    let target = std::env::var("TARGET").unwrap();
    if target != "wasm32-unknown-emscripten" {
        panic!("This project only supports wasm32-unknown-emscripten target. Use: cargo build --target wasm32-unknown-emscripten");
    }

    println!("cargo:warning=Building for WebAssembly (Emscripten)");

    // Use local assets directory for libraries
    let assets_dir = std::env::current_dir()
        .unwrap()
        .join("assets");

    // Link PDFium and QPDF static libraries from assets
    println!("cargo:rustc-link-search=native={}", assets_dir.display());
    println!("cargo:rustc-link-lib=static=pdfium");
    println!("cargo:rustc-link-lib=static=qpdf");

    // Compile C++ files:
    // 1. stub.cpp - minimal C++ stub for C++ runtime
    let headers_dir = assets_dir.join("include");

    cc::Build::new()
        .cpp(true)
        .file("src/stub.cpp")
        .include(&headers_dir)
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-frtti")
        .flag_if_supported("-fexceptions")
        .compile("cpp_stub");

    println!("cargo:warning=Build configuration:");
    println!("cargo:warning=  Libraries: {}", assets_dir.display());
    println!("cargo:warning=  Headers: {}", headers_dir.display());
    println!("cargo:warning=  libpdfium.a: 19 MB (includes StreamingIO functions)");
    println!("cargo:warning=  libqpdf.a: 7 MB");
    println!("cargo:warning=  NOTE: Calling PDFium C API directly (no wrappers needed)");
}
