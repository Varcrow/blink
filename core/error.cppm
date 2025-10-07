export module stfm.error;

namespace stfm {

export enum class Error {
    None,
    IncorrectArgs,
    MissingArgs,
    Failed,
    FileExists,
    DirExists,
};

} // namespace stfm
