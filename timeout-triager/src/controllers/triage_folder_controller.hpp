#pragma once

#include <string>
#include <vector>
#include "../model/training_files_model.hpp"

class TriageFolderController {
public:
  TriageFolderController(const std::string& directory_path);
  void setDirectoryPath(const std::string &directory_path);
  std::vector<TrainingFileModel> getTrainingFileModels();

private:
    std::string directory_path;
};
