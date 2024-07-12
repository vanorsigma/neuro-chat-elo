#pragma once

#include <cassert>
#include <sndfile.h>
#include <string>
#include <vector>
#include <wx/cmdline.h>
#include <wx/wx.h>

namespace audio {
std::vector<float> squeeze(std::vector<float> data);
std::vector<float> getWaveFormForAudioFile(const char *filename);
} // namespace audio

namespace Options {
static const wxCmdLineEntryDesc cmdLineDesc[] = {
    {wxCMD_LINE_SWITCH, "h", "help", "This message", wxCMD_LINE_VAL_NONE,
     wxCMD_LINE_OPTION_HELP},
    {wxCMD_LINE_OPTION, "t", "triage", "triage directory", wxCMD_LINE_VAL_STRING},
    {wxCMD_LINE_OPTION, "n", "neuro", "neuro directory", wxCMD_LINE_VAL_STRING},
    {wxCMD_LINE_OPTION, "e", "evil", "evil directory", wxCMD_LINE_VAL_STRING},
    {wxCMD_LINE_OPTION, "x", "none", "none directory", wxCMD_LINE_VAL_STRING},
    wxCMD_LINE_DESC_END};

struct Options {
  const std::string triage_directory;
  const std::string neuro_directory;
  const std::string evil_directory;
  const std::string none_directory;

  static const Options parse_from_cmdline();

protected:
  Options(const std::string &triage, const std::string &neuro,
          const std::string &evil, const std::string &none);
};
} // namespace Options
