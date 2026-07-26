cmake_minimum_required(VERSION 3.15...3.26)
project(lotrc LANGUAGES C)

include_directories(${CMAKE_CURRENT_SOURCE_DIR})

find_package(
  Python
  COMPONENTS Interpreter Development.Module NumPy
  REQUIRED)

add_custom_command(
  OUTPUT lotrc.c
  COMMAND Python::Interpreter -m cython
          "${CMAKE_CURRENT_SOURCE_DIR}/lotrc.pyx" --output-file lotrc.c
  DEPENDS lotrc.pyx
  VERBATIM)

add_compile_options(-D Py_LIMITED_API=0x030B0000)
add_compile_definitions(Py_LIMITED_API=0x030B0000)
python_add_library(lotrc MODULE lotrc.c WITH_SOABI)

target_link_libraries(lotrc PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../target/release/liblotrc_ffi.a Python::NumPy)

install(TARGETS lotrc DESTINATION .)
