# swig

Generate bindings between C/C++ code and various high level languages such as JavaScript, Python, C#, and more. It uses special `.i` or `.swg` files to generate the bindings (C/C++ with SWIG directives), then outputs a C/C++ file that contains all the wrapper code needed to build an extension module.

## Usage
- `swig -c++ -python -o path/to/output_wrapper.cpp path/to/swig_file.i` : generate a binding between C++ and Python
- `swig -go -cgo -intgosize 64 -c++ path/to/swig_file.i` : generate a binding between C++ and Go
- `swig -java path/to/swig_file.i` : generate a binding between C and Java
- `swig -ruby -prefix "foo::bar::" path/to/swig_file.i` : generate a binding between C and Ruby and prefix the Ruby module with `foo::bar::`

## Categories
files, development, system

## Links
- More information: https://www.swig.org/Doc4.4/SWIGDocumentation.html#SWIG_nn2
- tldr page: https://tldr.inbrowser.app/pages/common/swig
