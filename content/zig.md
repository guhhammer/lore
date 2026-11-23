# zig

The Zig compiler and toolchain.

## Usage
- `zig build` : compile the project in the current directory
- `zig build run` : compile and run the project in the current directory
- `zig init` : initialize a `zig build` project with library and executable
- `zig test path/to/file.zig` : create and run a test build
- `zig build run -fwine -Dtarget=x86_64-windows` : cross compile, build, and run a project for `x86_64` architecture and `windows` operating system
- `zig fmt path/to/file.zig` : reformat Zig source into canonical form
- `zig translate-c -lc path/to/file.c` : translate a C file to `zig`
- `zig c++ path/to/file.cpp` : use Zig as a drop-in C++ compiler

## Categories
development, files

## Links
- More information: https://ziglang.org/documentation/master/
- tldr page: https://tldr.inbrowser.app/pages/common/zig
