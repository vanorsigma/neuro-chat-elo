#pragma once

#include <memory>
#include <wx/wx.h>
#include "../utils.hpp"

class TimeoutTriagerCMDLineDelegate {
public:
  virtual void onInitCmdLine(wxCmdLineParser &parser) = 0;
  virtual void onCmdLineParsed(wxCmdLineParser& parser) = 0;
};

class TimeoutTriagerControllerDelegate {
public:
  virtual void onViewShown() = 0;
  virtual void onNeuroConfirm() = 0;
  virtual void onEvilConfirm() = 0;
  virtual void onNext() = 0;
  virtual void onPrev() = 0;
};

class TimeoutTriagerController : public TimeoutTriagerControllerDelegate, public TimeoutTriagerCMDLineDelegate {
public:
  void onViewShown() override;
  void onNeuroConfirm() override;
  void onEvilConfirm() override;
  void onNext() override;
  void onPrev() override;

  void onInitCmdLine(wxCmdLineParser& parser) override;
  void onCmdLineParsed(wxCmdLineParser& parser) override;

private:
    std::unique_ptr<Options::Options> options;
    const Options::Options* const get_options();
};
