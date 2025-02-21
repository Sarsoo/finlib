include(ExternalProject)

project(finlib C CXX)

set(SOURCE_FILES
        finlib.cpp
)

add_library(finlib SHARED STATIC ${SOURCE_FILES})
add_library(finlib::finlib ALIAS finlib)

ExternalProject_Add(
        finlibrs
        DOWNLOAD_COMMAND ""
        CONFIGURE_COMMAND ""
        BUILD_COMMAND cargo build
        COMMAND cargo build --release
        BINARY_DIR "${CMAKE_SOURCE_DIR}/../"
        INSTALL_COMMAND ""
        LOG_BUILD ON)

add_dependencies(finlib finlibrs)
target_link_libraries(finlib
        debug "${CMAKE_SOURCE_DIR}/../target/debug/libfinlib_ffi.a"
        optimized "${CMAKE_SOURCE_DIR}/../target/release/libfinlib_ffi.a"
)

install(TARGETS finlib DESTINATION ${FINLIB_INSTALL_LIB_DIR})
install(FILES finlib.hpp DESTINATION ${FINLIB_INSTALL_INCLUDE_DIR})
