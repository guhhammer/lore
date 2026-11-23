# c++

This command is an alias of `g++`.

## Usage
- `g++ path/to/source1.cpp path/to/source2.cpp ... -o path/to/output_executable` : compile a source code file into an executable binary
- `g++ path/to/source.cpp -Wall -o path/to/output_executable` : activate output of all errors and warnings
- `g++ path/to/source.cpp -Wall -g -Og -o path/to/output_executable` : show common warnings, debug symbols in output, and optimize without affecting debugging
- `g++ path/to/source.cpp -std=c++98|c++11|c++14|c++17 -o path/to/output_executable` : choose a language standard to compile for (C++98/C++11/C++14/C++17)
- `g++ path/to/source.cpp -o path/to/output_executable -Ipath/to/header -Lpath/to/library -llibrary_name` : include libraries located at a different path than the source file
- `g++ -c path/to/source1.cpp path/to/source2.cpp ... && g++ -o path/to/output_executable path/to/source1.o path/to/source2.o ...` : compile and link multiple source code files into an executable binary
- `g++ path/to/source.cpp -O1|2|3|fast -o path/to/output_executable` : optimize the compiled program for performance
- `g++ --version` : display version

## Categories
files, development, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/c++
