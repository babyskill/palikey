cmake_minimum_required(VERSION 3.16)
project(ibus-palikey VERSION 0.1.0 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

include(GNUInstallDirs)

find_package(PkgConfig REQUIRED)
pkg_check_modules(IBUS REQUIRED ibus-1.0 gio-2.0 gobject-2.0 glib-2.0)

# Paths for PaliKey Rust engine
set(PALIKEY_ENGINE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../palikey-engine" CACHE PATH "Path to palikey-engine source directory")
set(PALIKEY_TARGET_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../target" CACHE PATH "Path to Cargo target directory")

# Locate static library libpalikey_engine.a
find_library(PALIKEY_ENGINE_LIB
    NAMES palikey_engine libpalikey_engine.a
    PATHS
        "${PALIKEY_TARGET_DIR}/release"
        "${PALIKEY_TARGET_DIR}/debug"
        "${CMAKE_CURRENT_SOURCE_DIR}/lib"
    NO_DEFAULT_PATH
)

if(NOT PALIKEY_ENGINE_LIB)
    message(STATUS "PaliKey engine static library not found in prebuilt directories. Attempting to build with Cargo...")
    find_program(CARGO_BIN cargo)
    if(CARGO_BIN)
        execute_process(
            COMMAND ${CARGO_BIN} build --release --manifest-path "${PALIKEY_ENGINE_DIR}/Cargo.toml"
            RESULT_VARIABLE CARGO_RES
        )
        if(CARGO_RES EQUAL 0)
            find_library(PALIKEY_ENGINE_LIB
                NAMES palikey_engine libpalikey_engine.a
                PATHS "${PALIKEY_TARGET_DIR}/release"
                NO_DEFAULT_PATH
            )
        endif()
    endif()
endif()

if(NOT PALIKEY_ENGINE_LIB)
    message(FATAL_ERROR "Could not find libpalikey_engine.a. Please build it first: cargo build --release -p palikey-engine")
endif()

# Executable target: ibus-engine-palikey
add_executable(ibus-engine-palikey
    src/main.c
    src/palikey_engine_ibus.c
    src/palikey_config.c
)

target_include_directories(ibus-engine-palikey PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${PALIKEY_ENGINE_DIR}/include
    ${IBUS_INCLUDE_DIRS}
)

target_compile_options(ibus-engine-palikey PRIVATE
    ${IBUS_CFLAGS_OTHER}
)

target_link_libraries(ibus-engine-palikey PRIVATE
    ${IBUS_LIBRARIES}
    ${PALIKEY_ENGINE_LIB}
    pthread
    dl
    m
)

# Component XML registration file
configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/data/palikey.xml.in"
    "${CMAKE_CURRENT_BINARY_DIR}/palikey.xml"
    @ONLY
)

# Installation rules
install(TARGETS ibus-engine-palikey
    DESTINATION "${CMAKE_INSTALL_LIBEXECDIR}"
)

install(FILES "${CMAKE_CURRENT_BINARY_DIR}/palikey.xml"
    DESTINATION "${CMAKE_INSTALL_DATADIR}/ibus/component"
)

install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/data/icons/palikey.svg"
    DESTINATION "${CMAKE_INSTALL_DATADIR}/icons/hicolor/scalable/apps"
)

# CPack Packaging (DEB & RPM)
set(CPACK_PACKAGE_NAME "ibus-palikey")
set(CPACK_PACKAGE_VENDOR "Giac Ngo")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "PaliKey Pāḷi Input Method for IBus")
set(CPACK_PACKAGE_VERSION_MAJOR ${PROJECT_VERSION_MAJOR})
set(CPACK_PACKAGE_VERSION_MINOR ${PROJECT_VERSION_MINOR})
set(CPACK_PACKAGE_VERSION_PATCH ${PROJECT_VERSION_PATCH})
set(CPACK_GENERATOR "DEB;RPM")
set(CPACK_DEBIAN_PACKAGE_MAINTAINER "Giac Ngo <info@giacngo.org>")
set(CPACK_DEBIAN_PACKAGE_DEPENDS "ibus (>= 1.5.0), libc6")
set(CPACK_DEBIAN_PACKAGE_SECTION "utils")
set(CPACK_RPM_PACKAGE_REQUIRES "ibus >= 1.5.0")
set(CPACK_RPM_PACKAGE_LICENSE "MIT")
set(CPACK_RPM_PACKAGE_GROUP "Applications/System")
include(CPack)
