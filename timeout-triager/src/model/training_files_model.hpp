#pragma once

#include <nlohmann/json.hpp>
#include <optional>
#include <string>
#include <wx/wx.h>

struct TrainingFileModel final {
  std::string sound_filename;
  bool detected;
  std::optional<std::string> username;
  float relative_timestamp;

  TrainingFileModel() = default;
  static TrainingFileModel from_json(const std::string &filename);

private:
  TrainingFileModel(wxString sound_filename, bool detected,
                    std::optional<wxString> username, float relative_timestamp);
};

// from the docs
namespace nlohmann {
template <typename T> struct adl_serializer<std::optional<T>> {
  static void to_json(json &j, const std::optional<T> &opt) {
    if (opt.has_value()) {
      j = opt.value();
    } else {
      j = nullptr;
    }
  }

  static void from_json(const json &j, std::optional<T> &opt) {
    if (j.is_null()) {
      opt = std::nullopt;
    } else {
      opt = j.template get<T>();
    }
  }
};
} // namespace nlohmann

NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(TrainingFileModel, sound_filename, detected,
                                   username, relative_timestamp)
