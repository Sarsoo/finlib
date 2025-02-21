cmake_minimum_required(VERSION 3.19)


if(${CMAKE_VERSION} VERSION_LESS 3.19)
    cmake_policy(VERSION ${CMAKE_MAJOR_VERSION}.${CMAKE_MINOR_VERSION})
endif()

include(FetchContent)

project(finlib-cpp)

set(CMAKE_CXX_STANDARD 23)

set(CMAKE_INSTALL_PREFIX ${PROJECT_SOURCE_DIR})

set(FINLIB_INSTALL_INCLUDE_DIR ${PROJECT_SOURCE_DIR}/include)
set(FINLIB_INSTALL_BIN_DIR ${PROJECT_SOURCE_DIR}/bin)
set(FINLIB_INSTALL_LIB_DIR ${PROJECT_SOURCE_DIR}/lib)

set(FINLIB_HEADERS_DIR ${PROJECT_SOURCE_DIR}/src/finlib)

include_directories(${FINLIB_INSTALL_INCLUDE_DIR})
include_directories(${FINLIB_HEADERS_DIR})

add_subdirectory(src)
add_subdirectory(test)
