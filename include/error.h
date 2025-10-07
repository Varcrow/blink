#pragma once

namespace stfm::error {

enum class Command {
    M,
    D,
    R,
    C,
    L,
};

enum class Error {
    IncorrectArgs,
    MissingArgs,
    Failed,
    PathExists,
};

} // namespace stfm::error
