# iverilog

Preprocess and compile Verilog HDL (IEEE-1364) code into executable programs for simulation.

## Usage
- `iverilog path/to/source.v -o path/to/executable` : compile a source file into an executable
- `iverilog path/to/source.v -Wall -o path/to/executable` : compile a source file into an executable while displaying all warnings
- `iverilog -o path/to/executable -tvvp path/to/source.v` : compile and run explicitly using the VVP runtime
- `iverilog path/to/source.v -o path/to/executable -Ipath/to/library_directory` : compile using Verilog library files from a different path
- `iverilog -E path/to/source.v` : preprocess Verilog code without compiling

## Categories
files, development

## Links
- More information: https://manned.org/iverilog
- tldr page: https://tldr.inbrowser.app/pages/common/iverilog
