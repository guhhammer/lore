# lib

The Microsoft Library Manager for creating and managing static libraries of object files.

## Usage
- `lib /OUT :path\to\library.lib path\to\file1.obj path\to\file2.obj ...` : create a static library from object files
- `lib /LIST path\to\library.lib` : list the contents of a library
- `lib path\to\library.lib path\to\file.obj` : add an object file to an existing library
- `lib /REMOVE :path\to\file.obj path\to\library.lib` : remove an object file from a library
- `lib /EXTRACT :path\to\file.obj path\to\library.lib` : extract an object file from a library
- `lib /DEF :path\to\definition.def /OUT:path\to\import.lib` : create an import library from a DLL

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, development, windows

## Links
- More information: https://learn.microsoft.com/cpp/build/reference/lib-reference
- tldr page: https://tldr.inbrowser.app/pages/windows/lib
