#include "constants.hpp"
#include "runtime.h"
#include <raylib.h>

int main() {
    rt::cli();
    SetTraceLogLevel(LOG_ERROR);
    SetConfigFlags(FLAG_WINDOW_RESIZABLE);
    InitWindow(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT, "basement");
    SetTargetFPS(constants::TARGET_FPS);

    while (!WindowShouldClose()) {
        BeginDrawing();
        ClearBackground(BLACK);
        EndDrawing();
    }

    CloseWindow();
}
