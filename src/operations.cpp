#include "operations.h"

#include <filesystem>
#include <fstream>

namespace {

namespace fs = std::filesystem;

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

} // namespace

namespace stfm::operations {

std::string Make(const std::string& type, const std::string& name) {
    if (type == "f") {
        return MakeFile(name);
    } else if (type == "d") {
        return MakeDirectory(name);
    }
    return "Unrecognized type " + type;
}

std::string Delete(const std::string& path) {
    if (fs::remove(path)) {
        return "Deleted " + path;
    }
    return "Failed to delete " + path;
}

// temporary linux style list
std::string List() {
    std::string list;
    for (auto& entry : fs::directory_iterator(fs::current_path())) {
        list.append(entry.path().string());
    }
    return list;
}

} // namespace stfm::operations
