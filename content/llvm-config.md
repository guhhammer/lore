# llvm-config

Get various configuration information needed to compile programs which use LLVM. Typically called from build systems, like in Makefiles or configure scripts.

## Usage
- `clang++ $(llvm-config --cxxflags --ldflags --libs) --output path/to/output_executable path/to/source.cc` : compile and link an LLVM based program
- `llvm-config --prefix` : print the `PREFIX` of your LLVM installation
- `llvm-config --targets-built` : print all targets supported by your LLVM build

## Categories
development, files

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-config.html
- tldr page: https://tldr.inbrowser.app/pages/common/llvm-config
