fn main() {
    println!("cargo:rerun-if-changed=src/bridge.cpp");
    println!("cargo:rerun-if-changed=src/bridge.h");
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

    // Compile bridge.cpp for WASM
    cc::Build::new()
        .cpp(true)
        .file("src/bridge.cpp")
        .include("src")
        .include(assets_dir.join("include"))
        .include(assets_dir.join("include/qpdf"))
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("pdfium_bridge");

    println!("cargo:warning=Build configuration:");
    println!("cargo:warning=  Libraries: {}", assets_dir.display());
    println!("cargo:warning=  libpdfium.a: 19 MB");
    println!("cargo:warning=  libqpdf.a: 7 MB");
}
