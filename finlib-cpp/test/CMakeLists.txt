cmake_minimum_required(VERSION 3.19)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(finlib_tests)

include_directories(${FINLIB_HEADERS_DIR})

set(SOURCE_FILES
    src/basic_test.cpp
)

include(FetchContent)
FetchContent_Declare(
        googletest
        GIT_REPOSITORY https://github.com/google/googletest
        GIT_TAG v1.16.0
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

add_executable(finlib_tests ${SOURCE_FILES})
target_link_libraries(finlib_tests finlib GTest::gtest_main)
install(TARGETS finlib_tests DESTINATION bin)

include(GoogleTest)
gtest_discover_tests(finlib_tests)