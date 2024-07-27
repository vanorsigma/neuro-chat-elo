project(timeout-triager)
cmake_minimum_required(VERSION 3.22)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17")

include(FetchContent)
FetchContent_Declare(json URL https://github.com/nlohmann/json/releases/download/v3.11.3/json.tar.xz)
FetchContent_MakeAvailable(json)

# source files
set(SOURCE_FILES
  src/main.cpp
  src/utils.cpp
  src/components/waveform_visualizer.cpp
  src/controllers/timeout_triager_controller.cpp
  src/controllers/triage_folder_controller.cpp
  src/controllers/triage_folder_controller.cpp
  src/model/training_files_model.cpp
)

# make an executable
add_executable(timeout-triager ${SOURCE_FILES})

# includes for wxwidgets and json
find_package(wxWidgets COMPONENTS base core)
if(wxWidgets_FOUND)
  include(${wxWidgets_USE_FILE})
  target_link_libraries(timeout-triager PRIVATE sndfile ${wxWidgets_LIBRARIES} nlohmann_json::nlohmann_json)
endif()
