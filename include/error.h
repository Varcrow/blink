#pragma once

namespace stfm::error {

enum class Error {
    None,
    IncorrectArgs,
    MissingArgs,
    Failed,
    FileExists,
    DirExists,
};

} // namespace stfm::error
