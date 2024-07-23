#include "training_files_model.hpp"
#include <cassert>
#include <fstream>
#include <nlohmann/json.hpp>

using namespace nlohmann;

TrainingFileModel TrainingFileModel::from_json(const std::string& filename) {
  std::ifstream input_file(filename);
  json data = json::parse(input_file);
  return data.template get<TrainingFileModel>();
}
