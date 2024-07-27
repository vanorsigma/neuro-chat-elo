#include "triage_folder_controller.hpp"

#include <atomic>
#include <filesystem>
#include <future>
#include <memory>
#include <nlohmann/json.hpp>
#include <thread>
#include <wx/progdlg.h>

TriageFolderController::TriageFolderController(
    const std::string &directory_path) {
  setDirectoryPath(directory_path);
}

void TriageFolderController::setParent(wxWindow *parent) {
  this->parent = parent;
}

void TriageFolderController::setDirectoryPath(
    const std::string &directory_path) {
  this->directory_path = directory_path;
}

std::vector<TrainingFileModel>::const_iterator
TriageFolderController::getTrainingFileModels() {
  if (this->directory_path.length() == 0 ||
      !this->training_file_models.empty()) {
    return this->training_file_models.cbegin();
  }

  assert(std::filesystem::exists(this->directory_path));

  std::atomic<bool> stop{false};
  wxProgressDialog progDialog("Loading training files...",
                              "Please wait while we load the training files.",
                              100, this->parent,
                              wxPD_APP_MODAL | wxPD_AUTO_HIDE);

  auto calculation = std::async(std::launch::async, [&]() {
    for (const auto &entry :
         std::filesystem::directory_iterator(this->directory_path)) {
      if (entry.path().extension() != ".json") {
        continue;
      }

      try {
        this->training_file_models.push_back(
            TrainingFileModel::from_json(entry.path()));
      } catch (nlohmann::detail::exception) {
        continue;
      }
    }
    stop.store(true);
    return this->training_file_models.cbegin();
  });

  progDialog.CenterOnParent();
  while (!stop.load()) {
    progDialog.Pulse();
    wxMilliSleep(100);
  }

  return calculation.get();
}

std::vector<TrainingFileModel>::const_iterator
TriageFolderController::getTrainingFileModelsEnd() {
  getTrainingFileModels();
  return this->training_file_models.cend();
}
