#include "operations/make.h"

#include "error.h"

#include <expected>
#include <filesystem>
#include <fstream>

namespace {

namespace fs = std::filesystem;

std::expected<void, stfm::Error> MakeFile(const std::string& name) {
    std::ofstream file(name);
    if (!file) {
        return std::unexpected(stfm::Error::Failed);
    }
    return {};
}

std::expected<void, stfm::Error> MakeDirectory(const std::string& name) {
    if (!fs::create_directories(name)) {
        return std::unexpected(stfm::Error::Failed);
    }
    return {};
}

} // namespace

namespace stfm::operations {

std::expected<void, stfm::Error> Make(const std::string& type, const std::string& name) {
    if (type == "f") {
        return MakeFile(name);
    } else if (type == "d") {
        return MakeDirectory(name);
    }
    return std::unexpected(stfm::Error::IncorrectArgs);
}

} // namespace stfm::operations
