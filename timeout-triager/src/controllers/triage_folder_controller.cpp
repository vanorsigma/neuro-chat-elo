#include "triage_folder_controller.hpp"

#include <filesystem>
#include <nlohmann/json.hpp>

TriageFolderController::TriageFolderController(
    const std::string &directory_path) {
  setDirectoryPath(directory_path);
}

void TriageFolderController::setDirectoryPath(
    const std::string &directory_path) {
  this->directory_path = directory_path;
}

std::vector<TrainingFileModel> TriageFolderController::getTrainingFileModels() {
  assert(std::filesystem::exists(this->directory_path));

  std::vector<TrainingFileModel> result;
  for (const auto &entry :
       std::filesystem::directory_iterator(this->directory_path)) {
    if (entry.path().extension() != ".json") {
      continue;
    }

    try {
        result.push_back(TrainingFileModel::from_json(entry.path()));
    } catch (nlohmann::detail::exception) {
      continue;
    }
  }

  return result;
}
