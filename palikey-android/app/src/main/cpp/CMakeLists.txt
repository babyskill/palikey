cmake_minimum_required(VERSION 3.22.1)

project("palikey_jni")

# Locate palikey-engine root and include headers
set(PALIKEY_ENGINE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../../../../palikey-engine")
set(PALIKEY_TARGET_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../../../../target")

include_directories(
    "${PALIKEY_ENGINE_DIR}/include"
)

# Map Android ABI to Rust target triple
if (${ANDROID_ABI} STREQUAL "arm64-v8a")
    set(RUST_TARGET "aarch64-linux-android")
elseif (${ANDROID_ABI} STREQUAL "armeabi-v7a")
    set(RUST_TARGET "armv7-linux-androideabi")
elseif (${ANDROID_ABI} STREQUAL "x86_64")
    set(RUST_TARGET "x86_64-linux-android")
else()
    message(FATAL_ERROR "Unsupported Android ABI: ${ANDROID_ABI}")
endif()

# Check release first, fallback to debug
set(RUST_STATIC_LIB "${PALIKEY_TARGET_DIR}/${RUST_TARGET}/release/libpalikey_engine.a")
if (NOT EXISTS "${RUST_STATIC_LIB}")
    set(RUST_STATIC_LIB "${PALIKEY_TARGET_DIR}/${RUST_TARGET}/debug/libpalikey_engine.a")
endif()

add_library(palikey_engine_static STATIC IMPORTED)
if (EXISTS "${RUST_STATIC_LIB}")
    set_target_properties(palikey_engine_static PROPERTIES
        IMPORTED_LOCATION "${RUST_STATIC_LIB}"
    )
endif()

add_library(palikey_jni SHARED
    native-lib.cpp
)

target_link_libraries(palikey_jni
    palikey_engine_static
    log
)
