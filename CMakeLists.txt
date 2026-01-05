cmake_minimum_required(VERSION 3.15)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE "Debug")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project("logic-gates")

set(EXTERNAL_DIR deps)
add_subdirectory(${EXTERNAL_DIR}/fmt)
add_subdirectory(${EXTERNAL_DIR}/raylib)
add_subdirectory(runtime ${CMAKE_BINARY_DIR}/runtime)

set(INCLUDE_FILES include)
file(GLOB_RECURSE SRC_FILES src/*.cpp)
add_executable(${PROJECT_NAME} ${SRC_FILES})
target_link_libraries(${PROJECT_NAME} PRIVATE
	raylib
	fmt::fmt
	runtime
)
target_include_directories(${PROJECT_NAME} PRIVATE
	${INCLUDE_FILES}
)
