# verilator

Convert Verilog and SystemVerilog hardware description language (HDL) design into a C++ or SystemC model to be executed after compiling.

## Usage
- `verilator --binary --build-jobs 0 -Wall path/to/source.v` : build a specific C project in the current directory
- `verilator --cc --exe --build --build-jobs 0 -Wall path/to/source.cpp path/to/output.v` : create a C++ executable in a specific folder
- `verilator --lint-only -Wall` : perform linting over a code in the current directory
- `verilator --xml-output -Wall path/to/output.xml` : create XML output about the design (files, modules, instance hierarchy, logic, and data types) to feed into other tools

## Categories
files, development, text

## Links
- More information: https://veripool.org/guide/latest/
- tldr page: https://tldr.inbrowser.app/pages/common/verilator
