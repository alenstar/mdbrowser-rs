# Build steps
cmake_minimum_required(VERSION 2.6)
project(cmark-wrapper)

#file(GLOB_RECURSE APP_SRC_FILES RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} *.c)
set(UTILS_SRC_FILES 
    cmark-wrapper/cmark.c
	)

include_directories(${CMAKE_CURRENT_SOURCE_DIR})
include_directories(
    ${PROJECT_SOURCE_DIR}/cmark-wrapper
	cmark/src 
	cmark/extensions
${CMAKE_CURRENT_BINARY_DIR}/cmark/src
${CMAKE_CURRENT_BINARY_DIR}/cmark/extensions
    )
set(CMARK_TESTS OFF CACHE BOOL "")
set(CMARK_STATIC ON CACHE BOOL "")
add_subdirectory(cmark)

#add_dependencies(utils tinyxml2)
add_library(cmark-wrapper SHARED ${UTILS_SRC_FILES})
target_link_libraries(cmark-wrapper libcmark-gfm_static libcmark-gfmextensions_static)
