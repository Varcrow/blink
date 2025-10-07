#include "parsing.h"

#include "operations.h"

#include <print>
#include <vector>

namespace stfm::parsing {

std::expected<Arg, Error> ParseArg(std::istringstream& iss) {
    Arg arg;
    if (!(iss >> arg)) {
        return std::unexpected<Error>("Missing argument");
    }
    return arg;
}

std::expected<ArgPair, Error> ParseArgPair(std::istringstream& iss) {
    std::string arg1, arg2;
    if (!(iss >> arg1)) {
        return std::unexpected<Error>("Missing first arg");
    }
    if (!(iss >> arg2)) {
        return std::unexpected<Error>("Missing second arg");
    }
    return std::make_pair(arg1, arg2);
}

/*
    And so all of my bullshit error checking begins here
    everything in lower levels returns a string to get pushed
    into a log and will be very fun to update in the future
*/
bool ParseInput(const std::string& input) {
    std::string token;
    std::istringstream iss(input);
    std::vector<std::string> log;
    while (iss >> token) {
        if (token == "q") {
            return false;
        } else if (token == "m") {
            if (auto argPair = ParseArgPair(iss)) {
                log.push_back(operations::Make(argPair->first, argPair->second));
            } else {
                log.push_back(argPair.error());
            }
        } else if (token == "d") {
            if (auto arg = ParseArg(iss)) {
                log.push_back(operations::Delete(*arg));
            } else {
                log.push_back(arg.error());
            }
        } else if (token == "l") {
            log.push_back(operations::List());
        } else if (token == "r") {
            // rename
        } else if (token == "c") {
            // copy
        } else {
            log.push_back("Command " + token + " not recognized");
        }
    }

    for (auto& message : log) {
        std::println("[log]> {}", message);
    }

    return true;
}

} // namespace stfm::parsing
