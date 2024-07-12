#include "timeout_triager_controller.hpp"
#include <memory>
#include <utility>

void TimeoutTriagerController::onViewShown() {
    get_options(); // initialize once
}

void TimeoutTriagerController::onNeuroConfirm() {
  auto options = get_options();
  wxPrintf("xdd %s\n", options->neuro_directory);
}

void TimeoutTriagerController::onEvilConfirm() {}

void TimeoutTriagerController::onNext() {}

void TimeoutTriagerController::onPrev() {}

const Options::Options *const TimeoutTriagerController::get_options() {
  if (!options) {
    options = std::unique_ptr<Options::Options>(
        new Options::Options(Options::Options::parse_from_cmdline()));
  }
  return options.get();
}
