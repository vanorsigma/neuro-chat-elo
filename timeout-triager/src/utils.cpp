#include "utils.hpp"

#include <iostream>
using namespace std;

const int SQUEEZE_NUMBER = 30;

std::vector<float> squeeze(std::vector<float> data) {
    std::vector<float> retVal;
    for (int i = 0; i < data.size(); i += SQUEEZE_NUMBER) {
      float sum = 0;
      for (int j = 0; j < SQUEEZE_NUMBER; j++) {
        sum += data[i + j];
      }
      retVal.push_back((double) sum / SQUEEZE_NUMBER);
    }

    return retVal;
}

std::vector<float> getWaveFormForAudioFile(const char* filename) {
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
