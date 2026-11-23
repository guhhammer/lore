# wasm-objdump

WebAssembly 바이너리의 정보를 표시.

## Usage
- `wasm-objdump -h 파일.wasm` : 주어진 바이너리의 섹션 헤더 표시
- `wasm-objdump -d 파일.wasm` : 주어진 바이너리의 전체 디스어셈블 출력 표시
- `wasm-objdump -x 파일.wasm` : 각 섹션의 세부 정보 표시
- `wasm-objdump -j 'import' -x 파일.wasm` : 주어진 섹션의 세부 정보 표시

## Links
- More information: https://webassembly.github.io/wabt/doc/wasm-objdump.1.html
