cmake_minimum_required(VERSION 3.0)
project(cfltk)

add_subdirectory(fltk)

add_library(cfltk SHARED cfl.cpp cfl_window.cpp cfl_button.cpp cfl_widget.cpp)
target_include_directories(cfltk PUBLIC fltk)
target_link_libraries(cfltk PUBLIC fltk)

