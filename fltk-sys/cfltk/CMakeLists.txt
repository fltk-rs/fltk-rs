cmake_minimum_required(VERSION 3.0)
project(cfltk)

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)

add_subdirectory(fltk)

add_library(cfltk STATIC 
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

target_include_directories(cfltk PUBLIC fltk)
target_include_directories(cfltk PUBLIC ${CMAKE_CURRENT_BINARY_DIR}/fltk)

# The following line is necessary if building a shared/dynamic library
# target_link_libraries(cfltk PUBLIC fltk fltk_images fltk_z fltk_jpeg fltk_png)
