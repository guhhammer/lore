# ltrace

Display dynamic library calls of a process.

## Usage
- `ltrace path/to/program` : print (trace) library calls of a program binary
- `ltrace -c path/to/program` : count library calls. Print a handy summary at the bottom
- `ltrace -e malloc+free-@libc.so* path/to/program` : trace calls to malloc and free, omit those done by libc
- `ltrace -o file path/to/program` : write to file instead of terminal

## Categories
files, development, desktop

## Links
- More information: https://manned.org/ltrace
- tldr page: https://tldr.inbrowser.app/pages/linux/ltrace
