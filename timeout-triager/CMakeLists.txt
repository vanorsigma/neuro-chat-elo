project(timeout-triager)
cmake_minimum_required(VERSION 3.22)
set(CMAKE_CXX_STANDARD 14)

# source files
set(SOURCE_FILES
  src/main.cpp
  src/utils.cpp
  src/components/waveform_visualizer.cpp
  src/controllers/timeout_triager_controller.cpp
)

# make an executable
add_executable(timeout-triager ${SOURCE_FILES})

# includes for wxwidgets
find_package(wxWidgets COMPONENTS base core)
if(wxWidgets_FOUND)
  include(${wxWidgets_USE_FILE})
  target_link_libraries(timeout-triager sndfile ${wxWidgets_LIBRARIES})
endif()
