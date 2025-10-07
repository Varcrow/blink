#pragma once

#include <string>

namespace stfm::operations {

std::string Make(const std::string& type, const std::string& name);
std::string Delete(const std::string& path);
std::string Rename(const std::string& name, std::string& newName);
std::string Copy(const std::string& oldPath, const std::string& newPath);
std::string List();

} // namespace stfm::operations
