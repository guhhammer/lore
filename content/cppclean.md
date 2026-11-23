# cppclean

Find unused code in C++ projects.

## Usage
- `cppclean path/to/project` : run in a project's directory
- `cppclean path/to/project --include-path inc1 --include-path inc2` : run on a project where the headers are in the `inc1/` and `inc2/` directories
- `cppclean main.cpp` : run on a specific file `main.cpp`
- `cppclean . --exclude build` : run on the current directory, excluding the "build" directory

## Categories
files, development

## Links
- More information: https://github.com/myint/cppclean
- tldr page: https://tldr.inbrowser.app/pages/common/cppclean
