# cppcheck

A static analysis tool for C/C++ code. Instead of syntax errors, it focuses on the types of bugs that compilers normally do not detect.

## Usage
- `cppcheck . 2>cppcheck.log` : recursively check the current directory, showing progress on the screen and logging error messages to a file
- `cppcheck -q path/to/directory` : recursively check a given directory, and don't print progress messages
- `cppcheck --enable error|warning|style|performance|portability|information|all path/to/file.cpp` : check a given file, specifying which tests to perform (by default only errors are shown)
- `cppcheck --errorlist` : list available tests
- `cppcheck --suppress test_id1 --suppress test_id2 path/to/file.cpp` : check a given file, ignoring specific tests
- `cppcheck -I path/to/include_directory_1 -I path/to/include_directory_2 .` : check the current directory, providing paths for include files located outside it (e.g. external libraries)
- `cppcheck --project path/to/project.sln` : check a Microsoft Visual Studio project (`*.vcxproj`) or solution (`*.sln`)

## Categories
files, development

## Links
- More information: https://manned.org/cppcheck
- tldr page: https://tldr.inbrowser.app/pages/common/cppcheck
- Wikipedia: https://en.wikipedia.org/wiki/Cppcheck
