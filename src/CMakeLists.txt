add_executable(${PROJECT_NAME} "main.cpp" "stfm.cpp" "parsing.cpp" "operations/make.cpp" "operations/remove.cpp")
target_include_directories(${PROJECT_NAME} PRIVATE "${CMAKE_SOURCE_DIR}/include")