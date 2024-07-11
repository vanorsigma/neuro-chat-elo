#include "controllers/timeout_triager_controller.hpp"
#include "utils.hpp"
#include "waveform_visualizer.hpp"
#include <cstdlib>
#include <ctime>
#include <memory>
#include <wx/gdicmn.h>
#include <wx/wx.h>

using namespace std;

static TimeoutTriagerController controller;
static shared_ptr<TimeoutTriagerControllerDelegate>
    delegate(static_cast<TimeoutTriagerControllerDelegate *>(&controller));

class TimeoutTriagerGUIApp : public wxApp {
public:
  bool OnInit() override;
};

// We're doing a dialog because I don't want to configure i3 to make
// this a floating window by default
class TimeoutTriagerDialog : public wxDialog {
public:
  TimeoutTriagerDialog(const wxString &title);
  void
  setDelegate(shared_ptr<TimeoutTriagerControllerDelegate> const &delegate);

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

  /* Other stuff */
  weak_ptr<TimeoutTriagerControllerDelegate> delegate;
};

wxIMPLEMENT_APP(TimeoutTriagerGUIApp);

bool TimeoutTriagerGUIApp::OnInit() {
  TimeoutTriagerDialog *frame = new TimeoutTriagerDialog("some title");
  frame->Show(true);
  frame->setDelegate(delegate);
  return true;
}

enum { BUTTON_Prev, BUTTON_Neuro, BUTTON_None, BUTTON_Evil, BUTTON_Next };

TimeoutTriagerDialog::TimeoutTriagerDialog(const wxString &title)
    : wxDialog(NULL, wxID_ANY, title, wxDefaultPosition, wxSize(800, 600)) {
  waveformVisualizer = new WaveformVisualizer(this);
  auto data = squeeze(getWaveFormForAudioFile("something.wav"));
  waveformVisualizer->setWaveformData(data);

  mainSizer = new wxBoxSizer(wxVERTICAL);
  buttonSizers = new wxBoxSizer(wxHORIZONTAL);

  mainSizer->Add(waveformVisualizer,
                 wxSizerFlags().Expand().Proportion(100).Border(wxALL, 10));
  mainSizer->Add(buttonSizers,
                 wxSizerFlags().Center().Proportion(1).Border(wxALL, 10));

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

  // extremely important that I capture "this" and not the delegate
  neuro_button->Bind(wxEVT_BUTTON, [this](wxCommandEvent &) {
    if (!this->delegate.expired()) {
      this->delegate.lock().get()->onNeuroConfirm();
    }
  });

  evil_button->Bind(wxEVT_BUTTON, [this](wxCommandEvent &) {
    if (!this->delegate.expired()) {
      this->delegate.lock().get()->onEvilConfirm();
    }
  });

  next_button->Bind(wxEVT_BUTTON, [this](wxCommandEvent &) {
    if (!this->delegate.expired()) {
      this->delegate.lock().get()->onNext();
    }
  });

  prev_button->Bind(wxEVT_BUTTON, [this](wxCommandEvent &) {
    if (!this->delegate.expired()) {
      this->delegate.lock().get()->onPrev();
    }
  });

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

void TimeoutTriagerDialog::setDelegate(
    const shared_ptr<TimeoutTriagerControllerDelegate> &delegate) {
  this->delegate = weak_ptr<TimeoutTriagerControllerDelegate>(delegate);
}
