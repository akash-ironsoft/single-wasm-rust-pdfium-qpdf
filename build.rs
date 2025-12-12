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

    // Compile minimal C++ stub to ensure C++ runtime is linked
    // PDFium and QPDF are C++ libraries that need C++ stdlib support
    cc::Build::new()
        .cpp(true)
        .file("src/stub.cpp")
        .flag_if_supported("-std=c++17")
        .compile("cpp_stub");

    println!("cargo:warning=Build configuration:");
    println!("cargo:warning=  Libraries: {}", assets_dir.display());
    println!("cargo:warning=  libpdfium.a: 19 MB");
    println!("cargo:warning=  libqpdf.a: 7 MB");
    println!("cargo:warning=  NOTE: Calling PDFium C API directly (minimal C++ stub for runtime)");
}
