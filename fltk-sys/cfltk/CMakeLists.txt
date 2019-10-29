cmake_minimum_required(VERSION 3.0)
project(cfltk)

add_subdirectory(fltk)

add_library(cfltk STATIC cfl.cpp cfl_window.cpp cfl_button.cpp cfl_widget.cpp cfl_group.cpp)
target_include_directories(cfltk PUBLIC fltk)
target_link_libraries(cfltk PUBLIC fltk)


