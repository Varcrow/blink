export module stfm;
import parsing;
import std;

namespace stfm {

export int Run() {
    bool running = true;
    while (running) {
        std::print("[stfm]> ");
        std::string buffer;
        std::getline(std::cin, buffer);
        running = parsing::ParseInput(buffer);
    }
    return 0;
}

} // namespace stfm
