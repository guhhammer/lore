# pabcnetcclear

Preprocess and compile PascalABC.NET source files.

## Usage
- `pabcnetcclear path\to\source_file.pas` : compile the specified source file into an executable with the same name
- `pabcnetcclear /Output:path\to\file.exe path\to\source_file.pas` : compile the specified source file into an executable with the specified name
- `pabcnetcclear /Debug:0|1 path\to\source_file.pas` : compile the specified source file into an executable with the same name along with/without debug information
- `pabcnetcclear /SearchDir:path\to\directory path\to\source_file.pas` : allow units to be searched in the specified path while compiling the source file into an executable with the same name
- `pabcnetcclear /Define:symbol path\to\source_file.pas` : compile the specified source file into an executable, defining a symbol

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, development, windows

## Links
- More information: https://pascalabc.net/
- tldr page: https://tldr.inbrowser.app/pages/windows/pabcnetcclear
