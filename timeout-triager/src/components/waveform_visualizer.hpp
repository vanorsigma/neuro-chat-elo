#pragma once

#include <wx/wx.h>
#include <vector>
#include <optional>
#include <memory>

class WaveformVisualizer : public wxPanel {
public:
  WaveformVisualizer(wxWindow *parent);
  void setWaveformData(const std::vector<float> &data);

protected:
  void OnPaint(wxPaintEvent &event);
  void OnLeftClick(wxMouseEvent &event);
  void OnRightClick(wxMouseEvent &event);

private:
  std::vector<float> m_waveform_data;
  std::unique_ptr<wxPoint> start;
  std::unique_ptr<wxPoint> end;
};
