cmake_minimum_required(VERSION 3.16)
project(fcitx5-palikey VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

include(GNUInstallDirs)

# Fcitx5 Core dependency
find_package(Fcitx5Core 5.0.0 REQUIRED)

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

# Fallback install dirs if not provided by Fcitx5Core macros
if(NOT DEFINED FCITX_INSTALL_ADDONDIR)
    set(FCITX_INSTALL_ADDONDIR "${CMAKE_INSTALL_LIBDIR}/fcitx5")
endif()
if(NOT DEFINED FCITX_INSTALL_PKGDATADIR)
    set(FCITX_INSTALL_PKGDATADIR "${CMAKE_INSTALL_DATADIR}/fcitx5")
endif()

# Fcitx5 Addon Module
add_library(palikey MODULE
    src/palikey_engine_addon.cpp
    src/palikey_state.cpp
    src/palikey_config.cpp
)

target_include_directories(palikey PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${PALIKEY_ENGINE_DIR}/include
)

target_link_libraries(palikey PRIVATE
    Fcitx5::Core
    ${PALIKEY_ENGINE_LIB}
    pthread
    dl
    m
)

set_target_properties(palikey PROPERTIES
    PREFIX ""
    OUTPUT_NAME "palikey"
)

# Configuration files
configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/data/palikey-addon.conf.in"
    "${CMAKE_CURRENT_BINARY_DIR}/palikey-addon.conf"
    @ONLY
)

configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/data/palikey.conf.in"
    "${CMAKE_CURRENT_BINARY_DIR}/palikey.conf"
    @ONLY
)

# Installation rules
install(TARGETS palikey DESTINATION "${FCITX_INSTALL_ADDONDIR}")
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/palikey-addon.conf"
    DESTINATION "${FCITX_INSTALL_PKGDATADIR}/addon"
    RENAME "palikey.conf"
)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/palikey.conf"
    DESTINATION "${FCITX_INSTALL_PKGDATADIR}/inputmethod"
    RENAME "palikey.conf"
)
install(FILES "${CMAKE_CURRENT_SOURCE_DIR}/data/icons/org.giacngo.palikey.svg"
    DESTINATION "${CMAKE_INSTALL_DATADIR}/icons/hicolor/scalable/apps"
)

# CPack Packaging (DEB & RPM)
set(CPACK_PACKAGE_NAME "fcitx5-palikey")
set(CPACK_PACKAGE_VENDOR "Giac Ngo")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "PaliKey Pāḷi Input Method for Fcitx5")
set(CPACK_PACKAGE_VERSION_MAJOR ${PROJECT_VERSION_MAJOR})
set(CPACK_PACKAGE_VERSION_MINOR ${PROJECT_VERSION_MINOR})
set(CPACK_PACKAGE_VERSION_PATCH ${PROJECT_VERSION_PATCH})
set(CPACK_GENERATOR "DEB;RPM")
set(CPACK_DEBIAN_PACKAGE_MAINTAINER "Giac Ngo <info@giacngo.org>")
set(CPACK_DEBIAN_PACKAGE_DEPENDS "fcitx5 (>= 5.0.0), libc6")
set(CPACK_DEBIAN_PACKAGE_SECTION "utils")
set(CPACK_RPM_PACKAGE_REQUIRES "fcitx5 >= 5.0.0")
set(CPACK_RPM_PACKAGE_LICENSE "MIT")
set(CPACK_RPM_PACKAGE_GROUP "Applications/System")
include(CPack)
