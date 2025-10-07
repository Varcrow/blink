#pragma once
#include <expected>
#include <string>

// forward declaration
namespace stfm {
enum class Error;
}

namespace stfm::operations {
std::expected<void, stfm::Error> Remove(const std::string& path);
}
