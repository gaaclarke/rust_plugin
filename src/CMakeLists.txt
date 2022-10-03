# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(rust_plugin_library VERSION 0.0.1 LANGUAGES C)

# A library toplevel library has to exist or gradle will will skil the
# `add_custom_target`.
add_library(rust_plugin_mock SHARED
 "rust_plugin.c"
)

if(${ANDROID_ABI} STREQUAL "arm64-v8a")
set(RUST_TARGET "aarch64-linux-android")
elseif(${ANDROID_ABI} STREQUAL "armeabi-v7a")
set(RUST_TARGET "armv7-linux-androideabi")
elseif(${ANDROID_ABI} STREQUAL "x86")
set(RUST_TARGET "i686-linux-android")
elseif(${ANDROID_ABI} STREQUAL "x86_64")
set(RUST_TARGET "x86_64-linux-android")
else()
message( FATAL_ERROR "unsupported abi ${ANDROID_ABI}" )
endif()

add_custom_target(rust_plugin ALL
  COMMAND cargo ndk -t ${ANDROID_ABI} build --release && cp "${CMAKE_SOURCE_DIR}/target/${RUST_TARGET}/release/librust_plugin.so" ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/librust_plugin.so
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  BYPRODUCTS "${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/librust_plugin.so"
  VERBATIM)

add_dependencies(rust_plugin_mock rust_plugin)
