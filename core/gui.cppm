module;

#include "ftxui/dom/node.hpp"
#include "ftxui/screen/color.hpp"

#include <ftxui/dom/elements.hpp>
#include <ftxui/dom/table.hpp>
#include <ftxui/screen/screen.hpp>


export module stfm.gui;

namespace stfm::gui {
using namespace ftxui;

auto table = ftxui::Table({
    {"file", "main.cpp"},
    {"dir", "include"},
});

export void print_table() {
    table.SelectAll().Border(LIGHT);

    // Add border around the first column.
    table.SelectColumn(0).Border(LIGHT);

    // Make first row bold with a double border.
    table.SelectRow(0).Decorate(bold);
    table.SelectRow(0).SeparatorVertical(LIGHT);
    table.SelectRow(0).Border(DOUBLE);

    // Align right the "Release date" column.
    table.SelectColumn(2).DecorateCells(align_right);

    // Select row from the second to the last.
    auto content = table.SelectRows(1, -1);
    // Alternate in between 3 colors.
    content.DecorateCellsAlternateRow(color(Color::Blue), 3, 0);
    content.DecorateCellsAlternateRow(color(Color::Cyan), 3, 1);
    content.DecorateCellsAlternateRow(color(Color::White), 3, 2);

    auto document = table.Render();
    auto screen = Screen::Create(Dimension::Fit(document, /*extend_beyond_screen=*/true));
    Render(screen, document);
    screen.Print();
}

} // namespace stfm::gui
