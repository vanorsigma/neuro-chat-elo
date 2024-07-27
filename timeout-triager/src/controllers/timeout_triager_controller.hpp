#pragma once

#include "../components/waveform_visualizer.hpp"
#include "../utils.hpp"
#include "triage_folder_controller.hpp"
#include <memory>
#include <wx/wx.h>

class TimeoutTriagerCMDLineDelegate {
public:
  virtual void onInitCmdLine(wxCmdLineParser &parser) = 0;
  virtual void onCmdLineParsed(wxCmdLineParser &parser) = 0;
};

class TimeoutTriagerControllerDelegate {
public:
  virtual void onViewShown() = 0;
  virtual void onNeuroConfirm() = 0;
  virtual void onEvilConfirm() = 0;
  virtual void onNext() = 0;
  virtual void onPrev() = 0;

  virtual void setParent(wxWindow *parent) = 0;
  virtual void setWaveformVisualizer(WaveformVisualizer *visualiser) = 0;
};

class TimeoutTriagerController : public TimeoutTriagerControllerDelegate,
                                 public TimeoutTriagerCMDLineDelegate {
public:
  void onViewShown() override;
  void onNeuroConfirm() override;
  void onEvilConfirm() override;
  void onNext() override;
  void onPrev() override;

  void setParent(wxWindow *parent) override;
  void setWaveformVisualizer(WaveformVisualizer *visualiser) override;

  void onInitCmdLine(wxCmdLineParser &parser) override;
  void onCmdLineParsed(wxCmdLineParser &parser) override;

private:
  std::unique_ptr<Options::Options> options;
  wxWindow *parent = nullptr;
  WaveformVisualizer *waveform_visualiser = nullptr;

  TriageFolderController folder_controller;
  TriageFolderController::const_iterator current_file;

  const Options::Options *const get_options();
};
