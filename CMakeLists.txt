cmake_minimum_required(VERSION 3.15)
project(pdfium_bridge CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# PDFium workspace paths
set(PDFIUM_ROOT "/home/akash/Dev/ironsoft/iron-universal/Universal.PdfEditor/pdfium-workspace/Universal.Pdfium")
set(PDFIUM_BUILD_DIR "${PDFIUM_ROOT}/out/linux-x64-release")

# Include directories
include_directories(
    ${PDFIUM_ROOT}/public
    ${PDFIUM_ROOT}/third_party/Universal.Qpdf/include
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

# Bridge library
add_library(pdfium_bridge STATIC
    src/bridge.cpp
)

# Note: We don't link PDFium here - it will be linked from Rust side
# This keeps the bridge library small and prevents symbol conflicts

# Set output directory to match Rust's expectations
set_target_properties(pdfium_bridge PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

# Install the library
install(TARGETS pdfium_bridge
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
)
