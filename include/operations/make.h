#pragma once
#include <expected>
#include <string>


// forward declaration
namespace stfm::error {
enum class Error;
}

namespace stfm::operations {
std::expected<void, stfm::error::Error> Make(const std::string& type, const std::string& name);
}
