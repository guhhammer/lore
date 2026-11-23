# wasm-opt

Optimize WebAssembly binary files.

## Usage
- `wasm-opt -O input.wasm -o output.wasm` : apply default optimizations and write to a given file
- `wasm-opt -O4 input.wasm -o output.wasm` : apply all optimizations and write to a given file (takes more time, but generates optimal code)
- `wasm-opt -Oz input.wasm -o output.wasm` : optimize a file for size
- `wasm-opt input.wasm --print` : print the textual representation of the binary to console

## Categories
files

## Links
- More information: https://manned.org/wasm-opt
- tldr page: https://tldr.inbrowser.app/pages/common/wasm-opt
