#include "timeout_triager_controller.hpp"
#include "triage_folder_controller.hpp"
#include <memory>
#include <utility>
#include <iostream>

void TimeoutTriagerController::onViewShown() {
  auto options = get_options();

  TriageFolderController training_data_outputs(options->triage_directory);
  for (auto &data : training_data_outputs.getTrainingFileModels()) {
      wxPrintf("sound filename %s\n", data.sound_filename);
  }
}

void TimeoutTriagerController::onNeuroConfirm() {}

void TimeoutTriagerController::onEvilConfirm() {}

void TimeoutTriagerController::onNext() {}

void TimeoutTriagerController::onPrev() {}

void TimeoutTriagerController::onInitCmdLine(wxCmdLineParser &parser) {
    parser.SetDesc(Options::cmdLineDesc);
}

void TimeoutTriagerController::onCmdLineParsed(wxCmdLineParser &parser) {
    options = std::unique_ptr<Options::Options>(
        new Options::Options(Options::Options::parse_from_cmdline(parser)));
}

const Options::Options *const TimeoutTriagerController::get_options() {
  if (!options) {
      options = std::make_unique<Options::Options>();
  }
  return options.get();
}
