export module stfm.parsing;
import stfm.error;
import stfm.make;
import stfm.remove;
import std;

namespace stfm::parsing {

using Arg = std::string;
using ArgPair = std::pair<std::string, std::string>;

std::expected<Arg, stfm::Error> ParseArg(std::istringstream& iss) {
    Arg arg;
    if (!(iss >> arg)) {
        return std::unexpected(stfm::Error::MissingArgs);
    }
    return arg;
}

std::expected<ArgPair, stfm::Error> ParseArgPair(std::istringstream& iss) {
    std::string arg1, arg2;
    if (!(iss >> arg1)) {
        return std::unexpected(stfm::Error::MissingArgs);
    }
    if (!(iss >> arg2)) {
        return std::unexpected(stfm::Error::MissingArgs);
    }
    return std::make_pair(arg1, arg2);
}

export bool ParseInput(const std::string& input) {
    std::string token;
    std::istringstream iss(input);
    // this vector is here to remind me that I actually need to do something with these soon:)
    std::vector<stfm::Error> errors;
    while (iss >> token) {
        if (token == "q") {
            return false;
        } else if (token == "m") {
            if (auto argPair = ParseArgPair(iss)) {
                auto result = operations::Make(argPair->first, argPair->second);
                if (!result) {
                    errors.push_back(result.error());
                }
            } else {
                errors.push_back(argPair.error());
            }
        } else if (token == "r") {
            if (auto arg = ParseArg(iss)) {
                auto result = operations::Remove(*arg);
                if (!result) {
                    errors.push_back(result.error());
                }
            } else {
                errors.push_back(arg.error());
            }
        } else {
        }
    }

    return true;
}

} // namespace stfm::parsing
