# pkg-config

Provide the details of installed libraries for compiling applications.

## Usage
- `pkg-config --libs library1 library2 ...` : get the list of libraries and their dependencies
- `pkg-config --cflags --libs library1 library2 ...` : get the list of libraries, their dependencies, and proper cflags for gcc
- `pkg-config --modversion module1 module2 ...` : print the version of the linked modules
- `c++ example.cpp $(pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0) -o example` : compile your code with libgtk-3, libwebkit2gtk-4.0, and all their dependencies

## Categories
packages

## Links
- More information: https://manned.org/pkg-config
- tldr page: https://tldr.inbrowser.app/pages/common/pkg-config
- Wikipedia: https://en.wikipedia.org/wiki/Pkg-config
