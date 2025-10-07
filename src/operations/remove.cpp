#include "operations/remove.h"

#include "error.h"

#include <expected>
#include <filesystem>

namespace fs = std::filesystem;

namespace stfm::operations {

std::expected<void, stfm::Error> Remove(const std::string& path) {
    if (!fs::remove_all(path)) {
        return std::unexpected(stfm::Error::Failed);
    }
    return {};
}

} // namespace stfm::operations
