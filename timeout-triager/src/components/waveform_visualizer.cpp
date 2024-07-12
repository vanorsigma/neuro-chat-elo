#include "waveform_visualizer.hpp"
#include <iostream>
#include <memory>
#include <wx/colour.h>
#include <wx/event.h>
#include <wx/gdicmn.h>
#include <wx/gtk/brush.h>
#include <wx/graphics.h>

const wxColor DARK_GREY_COLOR(100, 100, 100, 200);

WaveformVisualizer::WaveformVisualizer(wxWindow *parent)
    : wxPanel(parent, wxID_ANY) {
    this->Bind(wxEVT_PAINT, &WaveformVisualizer::OnPaint, this);
    this->Bind(wxEVT_LEFT_DOWN, &WaveformVisualizer::OnLeftClick, this);
    this->Bind(wxEVT_RIGHT_DOWN, &WaveformVisualizer::OnRightClick, this);
}

void WaveformVisualizer::setWaveformData(const std::vector<float> &data) {
    m_waveform_data = data;
    Refresh();
}

void WaveformVisualizer::OnLeftClick(wxMouseEvent &event) {
    auto logicalPosition = event.GetPosition();
    start = std::make_unique<wxPoint>(wxPoint(logicalPosition));
    Refresh();
}

void WaveformVisualizer::OnRightClick(wxMouseEvent &event) {
    auto logicalPosition = event.GetPosition();
    end = std::make_unique<wxPoint>(wxPoint(logicalPosition));
    Refresh();
}

void drawLineWithTwoPoints(wxGraphicsContext *gc, int x0, int y0, int x1,
                           int y1) {
    const wxPoint2DDouble points[] = {wxPoint2DDouble(x0, y0),
                                      wxPoint2DDouble(x1, y1)};
    gc->DrawLines(sizeof(points) / sizeof(wxPoint2DDouble), points);
}

void WaveformVisualizer::OnPaint(wxPaintEvent &event) {
  wxPaintDC dc(this);
  dc.Clear();

  wxGraphicsContext *gc = wxGraphicsContext::Create(dc);

  if (m_waveform_data.empty())
    return;

  wxSize size = GetClientSize();
  int width = size.GetWidth();
  int height = size.GetHeight();

  gc->SetPen(*wxBLACK_PEN);

  float xStep = static_cast<float>(width) / m_waveform_data.size();
  float centerY = height / 2.0;

  for (size_t i = 1; i < m_waveform_data.size(); ++i) {
    float x1 = (i - 1) * xStep;
    float y1 = centerY - m_waveform_data[i - 1] * centerY;
    float x2 = i * xStep;
    float y2 = centerY - m_waveform_data[i] * centerY;

    dc.DrawLine(x1, y1, x2, y2);
  }

  if (start) {
      gc->SetPen(*wxBLUE_PEN);
      drawLineWithTwoPoints(gc, start->x, 0, start->x, height);
  }

  if (end) {
      gc->SetPen(*wxRED_PEN);
      drawLineWithTwoPoints(gc, end->x, 0, end->x, height);
  }

  if (start && end) {
    gc->SetBrush(DARK_GREY_COLOR);
    gc->DrawRectangle(start->x, 0, end->x - start->x, height);
  }
}
