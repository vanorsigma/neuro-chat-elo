#include <wx/gdicmn.h>
#include <wx/wx.h>
#include <cstdlib>
#include <ctime>
#include "utils.hpp"
#include "waveform_visualizer.hpp"

using namespace std;

class TimeoutTriagerGUIApp : public wxApp {
public:
    bool OnInit() override;
};


class TestingFrame : public wxFrame {
public:
  TestingFrame(const wxString &title);

private:
    /* Sizers */
    wxBoxSizer *mainSizer;
    wxBoxSizer *buttonSizers;

    /* UI Components */
    WaveformVisualizer *waveformVisualizer;
    wxButton *prev_button;
    wxButton *evil_button;
    wxButton *none_button;
    wxButton *neuro_button;
    wxButton *next_button;
};

wxIMPLEMENT_APP(TimeoutTriagerGUIApp);

bool TimeoutTriagerGUIApp::OnInit() {
    TestingFrame *frame = new TestingFrame("some title");
    frame->Show(true);
    return true;
}

enum {
  BUTTON_Prev,
  BUTTON_Neuro,
  BUTTON_None,
  BUTTON_Evil,
  BUTTON_Next
};

TestingFrame::TestingFrame(const wxString &title)
    : wxFrame(NULL, wxID_ANY, title, wxDefaultPosition, wxSize(800, 600)) {
  waveformVisualizer = new WaveformVisualizer(this);
  auto data = squeeze(getWaveFormForAudioFile("something.wav"));
  waveformVisualizer->setWaveformData(data);

  mainSizer = new wxBoxSizer(wxVERTICAL);
  buttonSizers = new wxBoxSizer(wxHORIZONTAL);

  mainSizer->Add(waveformVisualizer, wxSizerFlags().Expand().Proportion(100).Border(wxALL, 10));
  mainSizer->Add(buttonSizers, wxSizerFlags().Center().Proportion(1).Border(wxALL, 10));

  prev_button = new wxButton(this, BUTTON_Prev, "Previous");
  neuro_button = new wxButton(this, BUTTON_Neuro, "Neuro");
  none_button = new wxButton(this, BUTTON_None, "None");
  evil_button = new wxButton(this, BUTTON_Evil, "Evil");
  next_button = new wxButton(this, BUTTON_Next, "Next");

  neuro_button->SetFont(wxFont().Bold());
  neuro_button->SetForegroundColour(*wxRED);
  none_button->SetFont(wxFont().Bold());
  evil_button->SetFont(wxFont().Bold());
  evil_button->SetForegroundColour(*wxBLUE);

  buttonSizers->Add(prev_button);
  buttonSizers->AddSpacer(10);
  buttonSizers->Add(neuro_button);
  buttonSizers->AddSpacer(10);
  buttonSizers->Add(none_button);
  buttonSizers->AddSpacer(10);
  buttonSizers->Add(evil_button);
  buttonSizers->AddSpacer(10);
  buttonSizers->Add(next_button);
  SetSizer(mainSizer);
}
