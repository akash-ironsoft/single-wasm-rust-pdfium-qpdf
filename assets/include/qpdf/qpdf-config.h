/* qpdf-config.h - Configuration for QPDF WASM build */

#ifndef QPDF_CONFIG_H
#define QPDF_CONFIG_H

/* Define to 1 if you have the <inttypes.h> header file. */
#define QPDF_HAVE_INTTYPES_H 1
#define HAVE_INTTYPES_H 1

/* Define to 1 if you have the <stdint.h> header file. */
#define QPDF_HAVE_STDINT_H 1
#define HAVE_STDINT_H 1

/* Define to 1 if you have the <stdlib.h> header file. */
#define QPDF_HAVE_STDLIB_H 1
#define HAVE_STDLIB_H 1

/* Define to 1 if you have the <string.h> header file. */
#define QPDF_HAVE_STRING_H 1
#define HAVE_STRING_H 1

/* Define to 1 if you have the <strings.h> header file. */
#define QPDF_HAVE_STRINGS_H 1
#define HAVE_STRINGS_H 1

/* Define to 1 if you have the <sys/types.h> header file. */
#define QPDF_HAVE_SYS_TYPES_H 1
#define HAVE_SYS_TYPES_H 1

/* Define to 1 if you have the <unistd.h> header file. */
#define QPDF_HAVE_UNISTD_H 1
#define HAVE_UNISTD_H 1

/* Native crypto provider */
#define USE_CRYPTO_NATIVE 1
#define QPDF_CRYPTO_NATIVE 1

/* Default crypto provider */
#define DEFAULT_CRYPTO "native"

/* Major version */
#define QPDF_VERSION_MAJOR 11

/* Minor version */
#define QPDF_VERSION_MINOR 9

/* Patch version */
#define QPDF_VERSION_PATCH 1

/* Full version string */
/* #define QPDF_VERSION "11.9.1" */  /* Commented out - version is defined in DLL.h */

/* Large file support */
#define _FILE_OFFSET_BITS 64
#define _LARGEFILE_SOURCE 1
#define _LARGEFILE64_SOURCE 1

#endif /* QPDF_CONFIG_H */
