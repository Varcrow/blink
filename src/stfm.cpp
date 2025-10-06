#include "stfm.h"

#include "parsing.h"

#include <iostream>
#include <print>
#include <string>


namespace stfm {

int Run() {
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
