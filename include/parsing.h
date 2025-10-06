#pragma once

#include <expected>
#include <sstream>

namespace stfm::parsing {

using ArgPair = std::pair<std::string, std::string>;
using Error = std::string;

bool ParseInput(const std::string& input);
std::expected<ArgPair, Error> ParseArgPair(std::istringstream& iss);

} // namespace stfm::parsing
