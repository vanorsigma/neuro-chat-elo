#include "utils.hpp"

#include <cstdlib>
#include <iostream>

using namespace std;

const int SQUEEZE_NUMBER = 30;

std::vector<float> audio::squeeze(std::vector<float> data) {
  std::vector<float> retVal;
  for (int i = 0; i < data.size(); i += SQUEEZE_NUMBER) {
    float sum = 0;
    for (int j = 0; j < SQUEEZE_NUMBER; j++) {
      sum += data[i + j];
    }
    retVal.push_back((double)sum / SQUEEZE_NUMBER);
  }

  return retVal;
}

std::vector<float> audio::getWaveFormForAudioFile(const char *filename) {
  SF_INFO sfInfo;
  SNDFILE *sndFile = sf_open(filename, SFM_READ, &sfInfo);
  assert(sndFile);

  sf_count_t numFrames = sfInfo.frames * sfInfo.channels;

  std::vector<float> buffer(numFrames);
  sf_count_t numRead = sf_read_float(sndFile, buffer.data(), numFrames);
  assert(numRead == numFrames);
  sf_close(sndFile);

  std::vector<float> waveformData(sfInfo.frames);
  for (int i = 0; i < sfInfo.frames; ++i) {
    waveformData[i] = buffer[i * sfInfo.channels + 1];
  }

  return waveformData;
}

void if_not_found_help_then_quit(wxCmdLineParser& parser,
                                 const wxString &option_name,
                                 wxString *variable) {
  if (!parser.Found(option_name, variable)) {
    wxPrintf("Option %s is required\n", option_name);
    parser.Usage();
    exit(1);
  }
}

Options::Options::Options() {}

Options::Options::Options(const std::string &triage, const std::string &neuro,
                          const std::string &evil, const std::string &none)
    : triage_directory(triage), neuro_directory(neuro), evil_directory(evil),
      none_directory(none) {}

const Options::Options Options::Options::parse_from_cmdline(wxCmdLineParser& parser) {
  wxString triage_directory, neuro_directory, evil_directory, none_directory;
  if_not_found_help_then_quit(parser, "t", &triage_directory);
  if_not_found_help_then_quit(parser, "n", &neuro_directory);
  if_not_found_help_then_quit(parser, "e", &evil_directory);
  if_not_found_help_then_quit(parser, "x", &none_directory);

  wxPrintf("Triage Directory: %s\n", triage_directory);
  wxPrintf("Neuro Directory: %s\n", neuro_directory);
  wxPrintf("Evil Directory: %s\n", evil_directory);
  wxPrintf("None Directory: %s\n", none_directory);

  return Options(
      triage_directory.ToStdString(), neuro_directory.ToStdString(),
      evil_directory.ToStdString(), none_directory.ToStdString());
}
