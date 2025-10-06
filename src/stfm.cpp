#include "stfm.h"

#include <expected>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <print>
#include <sstream>
#include <string>

namespace {

namespace fs = std::filesystem;
using ArgPair = std::pair<std::string, std::string>;
using Error = std::string;

namespace helpers {

std::string MakeFile(const std::string& name) {
    if (fs::exists(name)) {
        return "File " + name + " already exists";
    }
    std::ofstream file(name);
    if (!file) {
        return "Failed to create file " + name;
    }
    return "Created file " + name;
}

std::string MakeDirectory(const std::string& name) {
    if (fs::exists(name)) {
        return "Directory " + name + " because it already exists";
    }
    if (!fs::create_directory(name)) {
        return "Failed to create directory " + name;
    }
    return "Created directory " + name;
}

} // namespace helpers

namespace parsing {

std::expected<ArgPair, Error> ParseArgPair(std::istringstream& iss) {
    std::string arg1, arg2;
    if (!(iss >> arg1)) {
        return std::unexpected<Error>("Incomplete command");
    }
    if (!(iss >> arg2)) {
        return std::unexpected<Error>("Incomplete command");
    }
    return std::make_pair(arg1, arg2);
}

bool ParseInput(const std::string& input) {
    std::string token;
    std::istringstream iss(input);
    std::vector<std::string> log;
    while (iss >> token) {
        if (token == "q") {
            return false;
        } else if (token == "m") {
            log.push_back(Make(iss));
        } else if (token == "d") {
            // delete
        } else if (token == "l") {
            // list
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

} // namespace parsing

namespace operations {

std::string Make(std::istringstream& iss) {
    auto argPair = ParseArgPair(iss);
    if (argPair) {
        const auto& [type, name] = *argPair;
        if (type == "f") {
            return MakeFile(name);
        } else if (type == "d") {
            return MakeDirectory(name);
        }
    }
    return argPair.error();
}

} // namespace operations

} // namespace

namespace stfm {

int Run() {
    bool running = true;
    while (running) {
        std::print("[stfm]> ");
        std::string buffer;
        std::getline(std::cin, buffer);
        running = ParseInput(buffer);
    }
    return 0;
}

} // namespace stfm
