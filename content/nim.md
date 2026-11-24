# nim

The Nim compiler. Process, compile, and link Nim language source files.

## Usage
- `nim c path/to/file.nim` : compile a source file
- `nim c -r path/to/file.nim` : compile and run a source file
- `nim c -d:release path/to/file.nim` : compile a source file with release optimizations enabled
- `nim c -d:release --opt:size path/to/file.nim` : build a release binary optimized for low file size
- `nim doc path/to/file.nim` : generate HTML documentation for a module (output will be placed in the current directory)
- `nim check path/to/file.nim` : check a file for syntax and semantics

## Categories
files, development, processes

## Links
- More information: https://nim-lang.org/docs/nimc.html
- tldr page: https://tldr.inbrowser.app/pages/common/nim
- Wikipedia: https://en.wikipedia.org/wiki/Nim_(programming_language)
