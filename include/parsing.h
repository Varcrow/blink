#pragma once

#include <expected>
#include <sstream>

namespace stfm::parsing {

using Arg = std::string;
using ArgPair = std::pair<std::string, std::string>;
using Error = std::string;

bool ParseInput(const std::string& input);
std::expected<Arg, Error> ParseArg(std::istringstream& iss);
std::expected<ArgPair, Error> ParseArgPair(std::istringstream& iss);

} // namespace stfm::parsing
