# Build System Diagram (Mermaid)

You can view this diagram by pasting it into [Mermaid Live Editor](https://mermaid.live/) or viewing on GitHub.

## High-Level Build Flow

```mermaid
graph TD
    A[User: cargo build] --> B[Cargo reads Cargo.toml]
    B --> C[Cargo executes build.rs]
    C --> D[CMake Build Stage]
    C --> E[autocxx Stage]
    D --> F[Link Instructions]
    E --> F
    F --> G[Compile Rust Code]
    G --> H[Link Everything]
    H --> I[Final Binary]

    style A fill:#e1f5ff
    style C fill:#fff3e0
    style D fill:#f3e5f5
    style E fill:#e8f5e9
    style I fill:#c8e6c9
```

## Detailed Build Process

```mermaid
sequenceDiagram
    participant User
    participant Cargo
    participant BuildRS as build.rs
    participant CMake
    participant AutoCXX as autocxx
    participant Compiler
    participant Linker

    User->>Cargo: cargo build
    Cargo->>Cargo: Read Cargo.toml
    Cargo->>BuildRS: Execute build script

    par CMake Build
        BuildRS->>CMake: cmake::Config::new(".")
        CMake->>CMake: Read CMakeLists.txt
        CMake->>CMake: Compile src/bridge.cpp
        CMake->>CMake: Create libpdfium_bridge.a
        CMake-->>BuildRS: Return build path
        BuildRS->>Cargo: cargo:rustc-link-lib=static=pdfium_bridge
    and autocxx Build
        BuildRS->>AutoCXX: Builder::new("src/lib.rs")
        AutoCXX->>AutoCXX: Parse bridge.h with clang
        AutoCXX->>AutoCXX: Generate Rust FFI code
        AutoCXX->>AutoCXX: Generate C++ glue code
        AutoCXX->>AutoCXX: Compile glue → libautocxx-pdfium-bridge.a
        AutoCXX-->>BuildRS: Return compile config
    end

    BuildRS->>Cargo: cargo:rustc-link-lib=dylib=pdfium
    BuildRS->>Cargo: cargo:rustc-link-arg=-Wl,-rpath,...
    BuildRS-->>Cargo: Done

    Cargo->>Compiler: Compile src/lib.rs
    Compiler->>Compiler: Expand autocxx::include_cpp!
    Compiler->>Compiler: Generate object files
    Compiler-->>Cargo: auto_pqdfium_rs.rlib

    Cargo->>Linker: Link all artifacts
    Linker->>Linker: Combine .rlib + .a files
    Linker->>Linker: Link .so libraries
    Linker-->>Cargo: Final binary

    Cargo-->>User: Build complete
```

## Component Architecture

```mermaid
graph LR
    subgraph "Rust Layer"
        A[src/lib.rs<br/>Public API] --> B[autocxx generated<br/>FFI bindings]
    end

    subgraph "Bridge Layer"
        B --> C[src/bridge.cpp<br/>C++ Bridge]
    end

    subgraph "PDFium Layer"
        C --> D[libpdfium.so<br/>PDFium + QPDF]
        D --> E[libc++.so<br/>Chromium C++]
    end

    subgraph "System Layer"
        E --> F[System Libraries<br/>pthread, dl, m]
    end

    style A fill:#e3f2fd
    style B fill:#fff3e0
    style C fill:#f3e5f5
    style D fill:#e8f5e9
    style E fill:#fce4ec
    style F fill:#efebe9
```

## Runtime Call Flow

```mermaid
sequenceDiagram
    participant App as User Application
    participant Rust as Rust API<br/>(lib.rs)
    participant FFI as autocxx FFI<br/>(generated)
    participant Bridge as C++ Bridge<br/>(bridge.cpp)
    participant PDFium as libpdfium.so

    App->>Rust: extract_text(&pdf_bytes)
    Rust->>Rust: initialize() [once]
    Rust->>Rust: Validate input
    Rust->>FFI: ffi::pdfium_bridge_extract_text(ptr, len)
    FFI->>Bridge: pdfium_bridge_extract_text(ptr, len)

    Bridge->>PDFium: FPDF_LoadMemDocument(data, size)
    PDFium-->>Bridge: FPDF_DOCUMENT handle

    Bridge->>PDFium: FPDF_GetPageCount(doc)
    PDFium-->>Bridge: page_count

    loop For each page
        Bridge->>PDFium: FPDF_LoadPage(doc, i)
        PDFium-->>Bridge: FPDF_PAGE handle

        Bridge->>PDFium: FPDFText_LoadPage(page)
        PDFium-->>Bridge: FPDF_TEXTPAGE handle

        Bridge->>PDFium: FPDFText_GetText(textpage, ...)
        PDFium-->>Bridge: UTF-16 text

        Bridge->>Bridge: Convert UTF-16 → UTF-8
    end

    Bridge->>Bridge: Allocate C string with malloc()
    Bridge-->>FFI: char* (C string)
    FFI-->>Rust: *mut c_char

    Rust->>Rust: CStr::from_ptr(ptr)
    Rust->>Rust: to_string_lossy()
    Rust->>FFI: ffi::pdfium_bridge_free_string(ptr)
    FFI->>Bridge: pdfium_bridge_free_string(ptr)
    Bridge->>Bridge: free(ptr)

    Rust-->>App: Result<String>
```

## Build Artifacts & Dependencies

```mermaid
graph TD
    subgraph "Source Files"
        A1[src/lib.rs]
        A2[src/error.rs]
        A3[src/bridge.cpp]
        A4[src/bridge.h]
        A5[CMakeLists.txt]
        A6[build.rs]
    end

    subgraph "Build Stage 1: CMake"
        A3 --> B1[bridge.cpp.o]
        B1 --> B2[libpdfium_bridge.a]
    end

    subgraph "Build Stage 2: autocxx"
        A1 --> C1[Parse autocxx::include_cpp!]
        A4 --> C1
        C1 --> C2[Generate FFI code]
        C2 --> C3[libautocxx-pdfium-bridge.a]
    end

    subgraph "Build Stage 3: rustc"
        A1 --> D1[Compile Rust]
        A2 --> D1
        C2 --> D1
        D1 --> D2[auto_pqdfium_rs.rlib]
    end

    subgraph "Build Stage 4: Linker"
        B2 --> E1[Link All]
        C3 --> E1
        D2 --> E1
        F1[libpdfium.so] --> E1
        F2[libc++.so] --> E1
        E1 --> E2[Final Binary]
    end

    subgraph "External Dependencies"
        F1
        F2
        F3[libpthread.so] --> E1
        F4[libdl.so] --> E1
    end

    style B2 fill:#f3e5f5
    style C3 fill:#e8f5e9
    style D2 fill:#e3f2fd
    style E2 fill:#c8e6c9
```

## File Dependencies Graph

```mermaid
graph TB
    subgraph "Public API"
        L1[lib.rs<br/>extract_text, pdf_to_json]
    end

    subgraph "Error Handling"
        E1[error.rs<br/>PdfiumError]
    end

    subgraph "FFI Layer"
        F1[autocxx generated<br/>ffi::pdfium_bridge_*]
    end

    subgraph "C++ Bridge"
        B1[bridge.h<br/>C declarations]
        B2[bridge.cpp<br/>C++ implementation]
    end

    subgraph "PDFium Headers"
        P1[fpdfview.h]
        P2[fpdf_text.h]
        P3[ipdf_qpdf.h]
    end

    subgraph "PDFium Library"
        P4[libpdfium.so]
    end

    L1 --> E1
    L1 --> F1
    F1 --> B1
    B1 --> B2
    B2 --> P1
    B2 --> P2
    B2 --> P3
    B2 -.runtime link.-> P4

    style L1 fill:#e3f2fd
    style E1 fill:#fff3e0
    style F1 fill:#e8f5e9
    style B1 fill:#f3e5f5
    style B2 fill:#f3e5f5
    style P4 fill:#ffccbc
```

## Build Configuration Flow

```mermaid
flowchart TD
    A[build.rs starts] --> B{Get PDFIUM_DIR}
    B -->|env var set| C[Use custom path]
    B -->|default| D[Use hardcoded path]

    C --> E[Run CMake]
    D --> E

    E --> F[CMake compiles bridge.cpp]
    F --> G[Output: libpdfium_bridge.a]

    A --> H[Setup autocxx]
    H --> I[Set include paths]
    I --> J[Parse src/lib.rs]
    J --> K[Generate bindings]
    K --> L[Compile glue code]

    G --> M[Emit link search paths]
    L --> M

    M --> N[Emit link libraries]
    N --> O[Emit rpath]
    O --> P[build.rs done]

    P --> Q[Cargo compiles Rust]
    Q --> R[Cargo links everything]
    R --> S[Final binary ready]

    style A fill:#e1f5ff
    style E fill:#f3e5f5
    style H fill:#e8f5e9
    style S fill:#c8e6c9
```

## Linker Resolution Order

```mermaid
graph LR
    A[Rust Code] -->|needs| B[ffi::pdfium_bridge_extract_text]
    B -->|provided by| C[libautocxx-pdfium-bridge.a]
    C -->|needs| D[pdfium_bridge_extract_text]
    D -->|provided by| E[libpdfium_bridge.a]
    E -->|needs| F[FPDF_LoadMemDocument]
    F -->|provided by| G[libpdfium.so]
    G -->|needs| H[std::__Cr::*]
    H -->|provided by| I[libc++.so]
    I -->|needs| J[pthread_create]
    J -->|provided by| K[libpthread.so]

    style A fill:#e3f2fd
    style C fill:#e8f5e9
    style E fill:#f3e5f5
    style G fill:#ffccbc
    style I fill:#fce4ec
    style K fill:#efebe9
```

## Build System Interaction Matrix

```mermaid
graph TD
    subgraph "Build Tools"
        T1[Cargo]
        T2[CMake]
        T3[autocxx]
        T4[rustc]
        T5[cc/clang]
        T6[linker]
    end

    subgraph "Configuration Files"
        C1[Cargo.toml]
        C2[build.rs]
        C3[CMakeLists.txt]
        C4[src/lib.rs]
    end

    T1 -->|reads| C1
    T1 -->|executes| C2
    C2 -->|invokes| T2
    C2 -->|invokes| T3
    T2 -->|reads| C3
    T3 -->|parses| C4
    T2 -->|invokes| T5
    T3 -->|invokes| T5
    T1 -->|invokes| T4
    T1 -->|invokes| T6
```
