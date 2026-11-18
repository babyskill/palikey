cmake_minimum_required(VERSION 3.15)
project(PaliKeyTSF VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_C_STANDARD 99)

# -----------------------------------------------------------------------------
# Architecture & Target Resolution
# -----------------------------------------------------------------------------
if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    set(RUST_TARGET "x86_64-pc-windows-msvc")
    set(PALIKEY_ARCH "x64")
else()
    set(RUST_TARGET "i686-pc-windows-msvc")
    set(PALIKEY_ARCH "Win32")
endif()

message(STATUS "PaliKey TSF Target Architecture: ${PALIKEY_ARCH} (${RUST_TARGET})")

# -----------------------------------------------------------------------------
# Sources & Headers
# -----------------------------------------------------------------------------
set(PALIKEY_TSF_SOURCES
    src/dll_main.cpp
    src/palikey_tsf.cpp
    src/class_factory.cpp
    src/context_state.cpp
    src/edit_session.cpp
    src/display_attribute.cpp
    src/register.cpp
    src/guids.cpp
    src/palikey_config.cpp
    src/PaliKeyTSF.def
)

if(WIN32)
    list(APPEND PALIKEY_TSF_SOURCES src/palikey_tsf.rc)
endif()

add_library(PaliKeyTSF SHARED ${PALIKEY_TSF_SOURCES})

# -----------------------------------------------------------------------------
# Include Directories
# -----------------------------------------------------------------------------
target_include_directories(PaliKeyTSF PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}/../palikey-engine/include
)

# -----------------------------------------------------------------------------
# Compiler & Linker Settings
# -----------------------------------------------------------------------------
if(MSVC)
    target_compile_options(PaliKeyTSF PRIVATE
        /W4
        /O2
        /utf-8
        /EHsc
    )
    # Statically link C/C++ runtime to produce zero-dependency DLL
    set_property(TARGET PaliKeyTSF PROPERTY
        MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>"
    )
endif()

set_target_properties(PaliKeyTSF PROPERTIES
    PREFIX ""
    OUTPUT_NAME "PaliKeyTSF"
    LINK_FLAGS "/DEF:\"${CMAKE_CURRENT_SOURCE_DIR}/src/PaliKeyTSF.def\""
)

# -----------------------------------------------------------------------------
# Engine Static Library & Win32 Dependencies
# -----------------------------------------------------------------------------
set(RUST_LIB_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../target/${RUST_TARGET}/release/palikey_engine.lib")
find_library(PALIKEY_ENGINE_LIB
    NAMES palikey_engine
    PATHS
        "${CMAKE_CURRENT_SOURCE_DIR}/../target/${RUST_TARGET}/release"
        "${CMAKE_CURRENT_SOURCE_DIR}/../target/${RUST_TARGET}/debug"
        "${CMAKE_CURRENT_SOURCE_DIR}/../target/release"
    NO_DEFAULT_PATH
)

if(PALIKEY_ENGINE_LIB)
    message(STATUS "Found palikey_engine library: ${PALIKEY_ENGINE_LIB}")
    target_link_libraries(PaliKeyTSF PRIVATE ${PALIKEY_ENGINE_LIB})
else()
    message(WARNING "palikey_engine.lib not found at ${RUST_LIB_PATH}. Remember to run 'cargo build --target ${RUST_TARGET} --release' in palikey-engine.")
    target_link_libraries(PaliKeyTSF PRIVATE "${RUST_LIB_PATH}")
endif()

# Win32 System Libraries
target_link_libraries(PaliKeyTSF PRIVATE
    advapi32
    ole32
    oleaut32
    user32
    shlwapi
    userenv
    bcrypt
)

# -----------------------------------------------------------------------------
# Install Target
# -----------------------------------------------------------------------------
install(TARGETS PaliKeyTSF
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)
