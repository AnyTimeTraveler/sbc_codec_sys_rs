cmake_minimum_required(VERSION 3.26)
project(sbc C)

set(CMAKE_C_STANDARD 11)

add_compile_definitions(HAVE_CONFIG_H)

#add_compile_options(-Wall -Wextra -Werror -Wno-unused-variable -Wno-unused-parameter -Wno-unused-function -Wno-div-by-zero)

include_directories(src)

add_library(sbc
        src/formats.h
        src/sbc.c
        src/sbc.h
        src/sbc_math.h
        src/sbc_primitives.c
        src/sbc_primitives.h
        src/sbc_primitives_armv6.c
        src/sbc_primitives_armv6.h
        src/sbc_primitives_iwmmxt.c
        src/sbc_primitives_iwmmxt.h
        src/sbc_primitives_mmx.c
        src/sbc_primitives_mmx.h
        src/sbc_primitives_neon.c
        src/sbc_primitives_neon.h
        src/sbc_tables.h
        src/sbcdec.c
        src/sbcenc.c
        src/sbcinfo.c
)
