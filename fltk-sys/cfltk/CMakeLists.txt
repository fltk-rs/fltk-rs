cmake_minimum_required(VERSION 3.0)
project(cfltk)

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)

option(CFLTK_BUILD_SHARED "Builds a shared library" OFF)
option(CFLTK_BUILD_TESTS "Builds C++ tests" OFF)

add_subdirectory(fltk)

set(CFLTK_SRCS  
    cfl.cpp 
    cfl_window.cpp 
    cfl_button.cpp 
    cfl_widget.cpp 
    cfl_group.cpp 
    cfl_text.cpp
    cfl_box.cpp 
    cfl_input.cpp 
    cfl_output.cpp
    cfl_menu.cpp
    cfl_dialog.cpp
    cfl_valuator.cpp
    cfl_browser.cpp
    cfl_misc.cpp
    cfl_image.cpp
    cfl_draw.cpp
    )

if(CFLTK_BUILD_SHARED)
    add_library(cfltk SHARED ${CFLTK_SRCS})
else()
    add_library(cfltk STATIC ${CFLTK_SRCS})
endif()

target_include_directories(cfltk PUBLIC fltk ${CMAKE_CURRENT_BINARY_DIR}/fltk)

if(CFLTK_BUILD_SHARED)
    target_link_libraries(cfltk PUBLIC fltk fltk_images fltk_z fltk_jpeg fltk_png)
endif()

if(CFLTK_BUILD_TESTS)
    enable_testing()
    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/tests)
    add_executable(test1 tests/test1.cpp)
    target_include_directories(test1 PUBLIC fltk ${CMAKE_CURRENT_BINARY_DIR}/fltk ${CMAKE_CURRENT_LIST_DIR})
    target_link_libraries(test1 PUBLIC cfltk fltk fltk_images fltk_z fltk_jpeg fltk_png)
    add_test(cfltk_tests test1)
endif()