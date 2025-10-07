#pragma once

namespace stfm {

enum class Error {
    None,
    IncorrectArgs,
    MissingArgs,
    Failed,
    FileExists,
    DirExists,
};

} // namespace stfm
