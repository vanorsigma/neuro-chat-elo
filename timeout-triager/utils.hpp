#pragma once

#include <cassert>
#include <vector>
#include <sndfile.h>

std::vector<float> squeeze(std::vector<float> data);
std::vector<float> getWaveFormForAudioFile(const char* filename);
