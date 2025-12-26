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

    // Compile C++ stub for C++ runtime support
    cc::Build::new()
        .cpp(true)
        .file("src/stub.cpp")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-frtti")
        .flag_if_supported("-fexceptions")
        .compile("cpp_stub");
}
