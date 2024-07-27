#pragma once

#include "../model/training_files_model.hpp"
#include <string>
#include <vector>

class TriageFolderController {
public:
  typedef std::vector<TrainingFileModel>::const_iterator const_iterator;

  TriageFolderController() = default;
  TriageFolderController(const std::string &directory_path);

  void setParent(wxWindow *parent);
  void setDirectoryPath(const std::string &directory_path);

  std::vector<TrainingFileModel>::const_iterator getTrainingFileModels();
  std::vector<TrainingFileModel>::const_iterator getTrainingFileModelsEnd();

private:
  wxWindow *parent;
  std::string directory_path;
  std::vector<TrainingFileModel> training_file_models;
};
