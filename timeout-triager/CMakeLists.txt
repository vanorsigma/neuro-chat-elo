project(timeout-triager)
cmake_minimum_required(VERSION 3.22)
set(CMAKE_CXX_STANDARD 14)

# source files
set(SOURCE_FILES
src/main.cpp
src/waveform_visualizer.cpp
src/utils.cpp
src/controllers/timeout_triager_controller.cpp)

# makex an executable
add_executable(timeout-triager ${SOURCE_FILES})

# includes for wxwidgets
find_package(wxWidgets COMPONENTS base core)
if(wxWidgets_FOUND)
  include(${wxWidgets_USE_FILE})
  target_link_libraries(timeout-triager sndfile ${wxWidgets_LIBRARIES})
endif()
