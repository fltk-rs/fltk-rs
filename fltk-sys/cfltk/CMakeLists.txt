cmake_minimum_required(VERSION 3.0)
project(cfltk)

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)

option(CFLTK_BUILD_SHARED "Builds a shared library" OFF)
option(CFLTK_BUILD_TESTS "Builds C++ tests" OFF)
option(USE_SYSTEM_FLTK "Finds and uses the installed FLTK library" OFF)
option(CFLTK_USE_OPENGL "Uses OpenGL" ON)

# If cmake can't find the installed fltk, you might have to set the FLTK_DIR enviromental variable,
# export FLTK_DIR=/path/to/fltk
if(USE_SYSTEM_FLTK)
    find_package(FLTK REQUIRED NO_MODULE)
else()
    add_subdirectory(fltk)
endif()

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
    cfl_table.cpp
    cfl_tree.cpp
    )

if(CFLTK_BUILD_SHARED)
    add_library(cfltk SHARED ${CFLTK_SRCS})
else()
    add_library(cfltk STATIC ${CFLTK_SRCS})
endif()

if(USE_SYSTEM_FLTK)
    target_include_directories(cfltk PUBLIC ${FLTK_INCLUDE_DIRS})
else()
    target_include_directories(cfltk PUBLIC fltk ${CMAKE_CURRENT_BINARY_DIR}/fltk)
endif()

set(CFLTK_LIBS fltk fltk_images fltk_z fltk_jpeg fltk_png)

if (CFLTK_USE_OPENGL)
    set(CFLTK_LIBS ${CFLTK_LIBS} fltk_gl)
    target_compile_definitions(cfltk PUBLIC CFLTK_USE_GL)
endif()

if(CFLTK_BUILD_SHARED OR USE_SYSTEM_FLTK)
    target_link_libraries(cfltk PUBLIC ${CFLTK_LIBS})
endif()

if(USE_SYSTEM_FLTK)
    install(TARGETS cfltk
        DESTINATION ${CMAKE_INSTALL_PREFIX}/lib
        )
endif()

if(CFLTK_BUILD_TESTS)
    enable_testing()
    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/tests)
    add_executable(test1 tests/test1.cpp)
    target_include_directories(test1 PUBLIC fltk ${CMAKE_CURRENT_BINARY_DIR}/fltk ${CMAKE_CURRENT_LIST_DIR})
    target_link_libraries(test1 PUBLIC cfltk ${CFLTK_LIBS})
    add_test(cfltk_tests test1)
endif()