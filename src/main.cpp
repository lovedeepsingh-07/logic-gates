#include "constants.hpp"
#include "runtime.h"
#include <raylib.h>

int main() {
    auto path_str = rt::cli();
    rt::watcher::run(std::string(path_str.begin(), path_str.end()));
    rt::logic::test();
    SetTraceLogLevel(LOG_ERROR);
    SetConfigFlags(FLAG_WINDOW_RESIZABLE);
    InitWindow(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT, "basement");
    SetTargetFPS(constants::TARGET_FPS);

    while (!WindowShouldClose()) {
        rt::watcher::poll_events();
        BeginDrawing();
        ClearBackground(BLACK);
        DrawText("hello, world!", 50, 50, 48, WHITE);
        EndDrawing();
    }

    CloseWindow();
}
