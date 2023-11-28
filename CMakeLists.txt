cmake_minimum_required(VERSION 3.26)
project(sbc C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)
set(CMAKE_VERBOSE_MAKEFILE on)

add_compile_definitions(HAVE_CONFIG_H)

include_directories(bluez_sbc)

add_library(sbc
        bluez_sbc/formats.h
        bluez_sbc/sbc.c
        bluez_sbc/sbc.h
        bluez_sbc/sbc_math.h
        bluez_sbc/sbc_primitives.c
        bluez_sbc/sbc_primitives.h
        bluez_sbc/sbc_tables.h
        bluez_sbc/sbcdec.c
        bluez_sbc/sbcenc.c
        bluez_sbc/sbcinfo.c
)
