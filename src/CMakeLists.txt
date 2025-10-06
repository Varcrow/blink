add_executable(${PROJECT_NAME} "main.cpp" "stfm.cpp" "parsing.cpp" "operations.cpp")
target_include_directories(${PROJECT_NAME} PRIVATE "${CMAKE_SOURCE_DIR}/include")