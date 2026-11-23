# gfortran

Preprocess and compile Fortran source files, then assemble and link them together.

## Usage
- `gfortran path/to/source1.f90 path/to/source2.f90 ... -o path/to/output_executable` : compile multiple source files into an executable
- `gfortran path/to/source.f90 -Wall -g -Og -o path/to/output_executable` : show common warnings, debug symbols in output, and optimize without affecting debugging
- `gfortran path/to/source.f90 -o path/to/output_executable -Ipath/to/mod_and_include -Lpath/to/library -llibrary_name` : include libraries from a different path
- `gfortran -S path/to/source.f90` : compile source code into Assembler instructions
- `gfortran -c path/to/source.f90` : compile source code into an object file without linking

## Categories
files, development

## Links
- More information: https://gcc.gnu.org/onlinedocs/gfortran/Invoking-GNU-Fortran.html
- tldr page: https://tldr.inbrowser.app/pages/common/gfortran
