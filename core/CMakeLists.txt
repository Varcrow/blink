add_library(stfm_lib)
target_sources(stfm_lib PUBLIC
    FILE_SET CXX_MODULES FILES
        stfm.cppm
        parsing.cppm
        remove.cppm
        make.cppm
        error.cppm
        gui.cppm
)
target_link_libraries(stfm_lib
  PRIVATE ftxui::screen
  PRIVATE ftxui::dom
  PRIVATE ftxui::component
)