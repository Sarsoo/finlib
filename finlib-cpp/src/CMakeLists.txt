cmake_minimum_required(VERSION 3.19)
project(finexe)

add_subdirectory(finlib)
set(SOURCE_FILES
        main.cpp
)

add_executable(finexe ${SOURCE_FILES})
target_link_libraries(finexe finlib)
install(TARGETS finexe DESTINATION ${FINLIB_INSTALL_BIN_DIR})