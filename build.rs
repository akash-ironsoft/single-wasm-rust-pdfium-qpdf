fn main() {
    let target = std::env::var("TARGET").unwrap();
    let is_wasm_emscripten = target == "wasm32-unknown-emscripten";
    let is_wasm_unknown = target == "wasm32-unknown-unknown";

    // Get PDFium directory from environment or use default
    let pdfium_dir = std::env::var("PDFIUM_DIR").unwrap_or_else(|_| {
        "/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium".to_string()
    });

    println!("cargo:rerun-if-env-changed=PDFIUM_DIR");
    println!("cargo:rerun-if-changed=src/bridge.cpp");
    println!("cargo:rerun-if-changed=src/bridge.h");
    println!("cargo:rerun-if-changed=src/lib.rs");

    if is_wasm_emscripten {
        build_for_wasm(&pdfium_dir);
    } else if is_wasm_unknown {
        build_for_wasm_unknown(&pdfium_dir);
    } else {
        println!("cargo:rerun-if-changed=CMakeLists.txt");
        build_for_native(&pdfium_dir);
    }
}

fn build_for_wasm_unknown(_pdfium_dir: &str) {
    println!("cargo:warning=Building for wasm32-unknown-unknown target");
    println!("cargo:warning=Note: C++ libraries cannot be linked with wasm32-unknown-unknown");
    println!("cargo:warning=The module will build but PDF functions will not work without C++ runtime");

    // For wasm32-unknown-unknown, we can't link C++ libraries
    // The build will succeed but functions will be stubs
    // This is mainly for demonstrating wasm-bindgen integration
}

fn build_for_wasm(pdfium_dir: &str) {
    println!("cargo:warning=Building for WebAssembly target");

    // Use Emscripten-built libraries
    let wasm_lib_dir = format!("{}/out/emscripten-wasm-release/obj", pdfium_dir);

    // Link PDFium and QPDF static libraries for WASM
    println!("cargo:rustc-link-search=native={}", wasm_lib_dir);
    println!("cargo:rustc-link-search=native={}/third_party/Universal.Qpdf", wasm_lib_dir);
    println!("cargo:rustc-link-lib=static=pdfium");
    println!("cargo:rustc-link-lib=static=qpdf");

    // Compile bridge.cpp for WASM using cc crate directly (no autocxx needed)
    // Let em++ use its default include paths for system headers
    cc::Build::new()
        .cpp(true)
        .file("src/bridge.cpp")
        .include("src")
        .include(format!("{}/public", pdfium_dir))
        .include(format!("{}/third_party/Universal.Qpdf/include", pdfium_dir))
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("pdfium_bridge");
}

fn build_for_native(pdfium_dir: &str) {
    println!("cargo:warning=Building for native target");

    // Build the C++ bridge library using CMake
    let dst = cmake::Config::new(".")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    // Link the CMake-built bridge library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=pdfium_bridge");

    // Link PDFium library (use shared build due to libc++ requirements)
    let pdfium_lib_dir = format!("{}/out/linux-x64-shared", pdfium_dir);
    println!("cargo:rustc-link-search=native={}", pdfium_lib_dir);
    println!("cargo:rustc-link-lib=dylib=pdfium");
    println!("cargo:rustc-link-lib=dylib=c++");

    // Add rpath so the shared libraries can be found at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", pdfium_lib_dir);

    // Set up autocxx for Rust bindings
    let include_paths = vec![
        "src".to_string(),
        format!("{}/public", pdfium_dir),
        format!("{}/third_party/Universal.Qpdf/include", pdfium_dir),
    ];

    let mut builder = autocxx_build::Builder::new("src/lib.rs", include_paths.iter().map(|s| s.as_str()))
        .build()
        .expect("Failed to build autocxx bindings");

    builder
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("autocxx-pdfium-bridge");

    // Link system libraries
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=m");
}
