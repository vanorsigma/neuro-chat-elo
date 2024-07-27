#include "timeout_triager_controller.hpp"
#include "triage_folder_controller.hpp"
#include <iostream>
#include <iterator>
#include <memory>
#include <utility>

void setWaveformDataForModel(
    WaveformVisualizer *waveform_visualizer, const std::string &prefix,
    const TriageFolderController::const_iterator iterator,
    const TriageFolderController::const_iterator end) {

  if (std::distance(iterator, end) == 0) {
    throw std::runtime_error("No models to set waveform data for");
  }

  auto model = *iterator;
  if (waveform_visualizer != nullptr) {
    wxPrintf("trying to load filename %s\n", model.sound_filename);
    auto data = audio::squeeze(audio::getWaveFormForAudioFile(
        (prefix + "/" + model.sound_filename).c_str()));
    wxPrintf("loaded filename %s\n", model.sound_filename);
    waveform_visualizer->setWaveformData(data);
  }
}

void TimeoutTriagerController::onViewShown() {
  auto options = get_options();
  folder_controller.setDirectoryPath(options->triage_directory);

  current_file = folder_controller.getTrainingFileModels();
  setWaveformDataForModel(this->waveform_visualiser, options->triage_directory,
                          current_file,
                          folder_controller.getTrainingFileModelsEnd());
}

void TimeoutTriagerController::onNeuroConfirm() {}

void TimeoutTriagerController::onEvilConfirm() {}

void TimeoutTriagerController::onNext() {
  if (current_file == folder_controller.getTrainingFileModelsEnd()) {
    return;
  }

  current_file++;
  setWaveformDataForModel(this->waveform_visualiser, options->triage_directory,
                          current_file,
                          folder_controller.getTrainingFileModelsEnd());
}

void TimeoutTriagerController::onPrev() {
  if (current_file == folder_controller.getTrainingFileModels()) {
    return;
  }

  current_file--;
  setWaveformDataForModel(this->waveform_visualiser, options->triage_directory,
                          current_file,
                          folder_controller.getTrainingFileModelsEnd());
}

void TimeoutTriagerController::setParent(wxWindow *parent) {
  this->parent = parent;
  this->folder_controller.setParent(parent);
};

void TimeoutTriagerController::setWaveformVisualizer(
    WaveformVisualizer *visualiser) {
  this->waveform_visualiser = visualiser;
}

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
