fn main() {
    // Get PDFium directory from environment or use default
    let pdfium_dir = std::env::var("PDFIUM_DIR").unwrap_or_else(|_| {
        "/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium".to_string()
    });

    println!("cargo:rerun-if-env-changed=PDFIUM_DIR");
    println!("cargo:rerun-if-changed=src/bridge.cpp");
    println!("cargo:rerun-if-changed=src/bridge.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=CMakeLists.txt");

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
