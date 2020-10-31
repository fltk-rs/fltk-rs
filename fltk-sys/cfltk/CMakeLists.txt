cmake_minimum_required(VERSION 3.0)
project(cfltk)

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(MSVC)
	string(REGEX REPLACE "/GR" "/GR-" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
    string(REGEX REPLACE "/EHsc" "/EHs-c-" CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
elseif(ANDROID)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -frtti")
else()
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-rtti -fno-exceptions")
endif()

if(UNIX AND NOT APPLE)
    add_definitions(-D_ALL_SOURCE)
endif()

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_INSTALL_PREFIX}/lib)

option(CFLTK_BUILD_SHARED "Builds a shared library" OFF)
option(CFLTK_BUILD_TESTS "Builds C++ tests" OFF)
option(USE_SYSTEM_FLTK "Finds and uses the installed FLTK library" OFF)
option(CFLTK_USE_OPENGL "Uses OpenGL" ON)
option(CFLTK_LINK_IMAGES "Links image libraries" ON)

# If cmake can't find the installed fltk, you might have to set the FLTK_DIR enviromental variable,
# export FLTK_DIR=/path/to/fltk
if(USE_SYSTEM_FLTK)
    find_package(FLTK REQUIRED NO_MODULE)
else()
    add_subdirectory(fltk)
endif()

set(CFLTK_SRCS
    src/cfl_new.cpp
    src/cfl.cpp 
    src/cfl_window.cpp 
    src/cfl_button.cpp 
    src/cfl_widget.cpp 
    src/cfl_group.cpp 
    src/cfl_text.cpp
    src/cfl_box.cpp 
    src/cfl_input.cpp 
    src/cfl_output.cpp
    src/cfl_menu.cpp
    src/cfl_dialog.cpp
    src/cfl_valuator.cpp
    src/cfl_browser.cpp
    src/cfl_misc.cpp
    src/cfl_image.cpp
    src/cfl_draw.cpp
    src/cfl_table.cpp
    src/cfl_tree.cpp
    src/cfl_surface.cpp
    )

if(CFLTK_BUILD_SHARED)
    add_library(cfltk SHARED ${CFLTK_SRCS})
else()
    add_library(cfltk STATIC ${CFLTK_SRCS})
endif()

if(USE_SYSTEM_FLTK)
    target_include_directories(cfltk PUBLIC include ${FLTK_INCLUDE_DIRS})
else()
    target_include_directories(cfltk PUBLIC include fltk ${CMAKE_CURRENT_BINARY_DIR}/fltk)
endif()

set(CFLTK_LIBS fltk)

if (CFLTK_USE_OPENGL)
    set(CFLTK_LIBS ${CFLTK_LIBS} fltk_gl)
    target_compile_definitions(cfltk PUBLIC CFLTK_USE_GL)
endif()

if (CFLTK_LINK_IMAGES)
    set(CFLTK_LIBS ${CFLTK_LIBS} fltk_images fltk_z fltk_jpeg fltk_png)
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