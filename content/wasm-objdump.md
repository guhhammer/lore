# wasm-objdump

Display information from WebAssembly binaries.

## Usage
- `wasm-objdump -h file.wasm` : display the section headers of a given binary
- `wasm-objdump -d file.wasm` : display the entire disassembled output of a given binary
- `wasm-objdump -x file.wasm` : display the details of each section
- `wasm-objdump -j 'import' -x file.wasm` : display the details of a given section

## Categories
desktop, files

## Links
- More information: https://webassembly.github.io/wabt/doc/wasm-objdump.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/wasm-objdump
