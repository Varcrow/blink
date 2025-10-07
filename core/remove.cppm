export module remove;
import error;
import std;

namespace stfm::operations {

namespace fs = std::filesystem;

export std::expected<void, stfm::Error> Remove(const std::string& path) {
    if (!fs::remove_all(path)) {
        return std::unexpected(stfm::Error::Failed);
    }
    return {};
}

} // namespace stfm::operations
