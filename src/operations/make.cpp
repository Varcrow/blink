#include "operations/make.h"

#include "error.h"

#include <expected>
#include <filesystem>
#include <fstream>

namespace {

namespace fs = std::filesystem;

std::expected<void, stfm::error::Error> MakeFile(const std::string& name) {
    if (fs::exists(name)) {
        return std::unexpected(stfm::error::Error::FileExists);
    }
    std::ofstream file(name);
    if (!file) {
        return std::unexpected(stfm::error::Error::Failed);
    }
    return {};
}

std::expected<void, stfm::error::Error> MakeDirectory(const std::string& name) {
    if (fs::exists(name)) {
        return std::unexpected(stfm::error::Error::DirExists);
    }
    if (!fs::create_directory(name)) {
        return std::unexpected(stfm::error::Error::Failed);
    }
    return {};
}

} // namespace

namespace stfm::operations {

std::expected<void, stfm::error::Error> Make(const std::string& type, const std::string& name) {
    if (type == "f") {
        return MakeFile(name);
    } else if (type == "d") {
        return MakeDirectory(name);
    }
    return std::unexpected(stfm::error::Error::IncorrectArgs);
}

} // namespace stfm::operations
