setup:
	nix run .#setup

build:
	@mkdir -p build
	@cmake -G Ninja -S . -B build
	@cmake --build ./build

run : build
	@nixGLIntel ./build/logic-gates ./demo
