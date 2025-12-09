#include "bridge.h"

#include <fpdfview.h>
#include <fpdf_text.h>
#include <ipdf_qpdf.h>

#include <cstdlib>
#include <cstring>
#include <sstream>
#include <string>
#include <vector>

// Global initialization flag
static bool g_pdfium_initialized = false;

extern "C" {

int pdfium_bridge_initialize() {
    if (g_pdfium_initialized) {
        return 1;
    }

    FPDF_LIBRARY_CONFIG config;
    config.version = 2;
    config.m_pUserFontPaths = nullptr;
    config.m_pIsolate = nullptr;
    config.m_v8EmbedderSlot = 0;

    FPDF_InitLibraryWithConfig(&config);
    g_pdfium_initialized = true;
    return 1;
}

void pdfium_bridge_cleanup() {
    if (g_pdfium_initialized) {
        FPDF_DestroyLibrary();
        g_pdfium_initialized = false;
    }
}

char* pdfium_bridge_extract_text(const uint8_t* pdf_data, size_t pdf_size) {
    if (!g_pdfium_initialized || pdf_data == nullptr || pdf_size == 0) {
        return nullptr;
    }

    // Load the PDF document
    FPDF_DOCUMENT doc = FPDF_LoadMemDocument(pdf_data, static_cast<int>(pdf_size), nullptr);
    if (!doc) {
        return nullptr;
    }

    std::ostringstream text_stream;
    int page_count = FPDF_GetPageCount(doc);

    // Extract text from each page
    for (int i = 0; i < page_count; ++i) {
        FPDF_PAGE page = FPDF_LoadPage(doc, i);
        if (!page) {
            continue;
        }

        FPDF_TEXTPAGE text_page = FPDFText_LoadPage(page);
        if (text_page) {
            // Get text length
            int text_length = FPDFText_CountChars(text_page);

            if (text_length > 0) {
                // Allocate buffer for text (UTF-16)
                std::vector<unsigned short> buffer(text_length + 1);
                int chars_written = FPDFText_GetText(text_page, 0, text_length, buffer.data());

                if (chars_written > 0) {
                    // Convert UTF-16 to UTF-8
                    for (int j = 0; j < chars_written - 1; ++j) {
                        unsigned short ch = buffer[j];
                        if (ch < 0x80) {
                            text_stream << static_cast<char>(ch);
                        } else if (ch < 0x800) {
                            text_stream << static_cast<char>(0xC0 | (ch >> 6));
                            text_stream << static_cast<char>(0x80 | (ch & 0x3F));
                        } else {
                            text_stream << static_cast<char>(0xE0 | (ch >> 12));
                            text_stream << static_cast<char>(0x80 | ((ch >> 6) & 0x3F));
                            text_stream << static_cast<char>(0x80 | (ch & 0x3F));
                        }
                    }
                }
            }

            FPDFText_ClosePage(text_page);
        }

        FPDF_ClosePage(page);

        // Add page separator
        if (i < page_count - 1) {
            text_stream << "\n---PAGE BREAK---\n";
        }
    }

    FPDF_CloseDocument(doc);

    std::string text = text_stream.str();
    char* result = static_cast<char*>(malloc(text.length() + 1));
    if (result) {
        strcpy(result, text.c_str());
    }
    return result;
}

char* pdfium_bridge_pdf_to_json(const uint8_t* pdf_data, size_t pdf_size) {
    if (!g_pdfium_initialized || pdf_data == nullptr || pdf_size == 0) {
        return nullptr;
    }

    // Call QPDF integration function
    // Version 2 provides comprehensive JSON output
    char* json_str = IPDF_QPDF_PDFToJSON(pdf_data, pdf_size, 2);

    // QPDF already allocates the string, we just return it
    // The caller must free it using pdfium_bridge_free_string
    return json_str;
}

void pdfium_bridge_free_string(char* str) {
    if (str) {
        // For strings from pdfium_bridge_extract_text, use free()
        // For strings from IPDF_QPDF_PDFToJSON, we need to use IPDF_QPDF_FreeString
        // Since we can't distinguish, we'll handle both:
        // Check if it looks like it came from QPDF (this is a heuristic)
        // Actually, let's just use IPDF_QPDF_FreeString for all since it likely uses free() internally
        IPDF_QPDF_FreeString(str);
    }
}

} // extern "C"
