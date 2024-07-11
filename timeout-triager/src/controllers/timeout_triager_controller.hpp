#pragma once

#include <wx/wx.h>

class TimeoutTriagerControllerDelegate {
public:
  virtual void onNeuroConfirm() = 0;
  virtual void onEvilConfirm() = 0;
  virtual void onNext() = 0;
  virtual void onPrev() = 0;
};

class TimeoutTriagerController : public TimeoutTriagerControllerDelegate {
  void onNeuroConfirm() override;
  void onEvilConfirm() override;
  void onNext() override;
  void onPrev() override;
};
