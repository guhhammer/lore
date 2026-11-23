# nmake

The Microsoft Program Maintenance Utility for building projects based on commands in a makefile.

## Usage
- `nmake` : build targets using the default makefile in the current directory
- `nmake /F path\to\makefile` : build targets using a specific makefile
- `nmake target` : build a specific target
- `nmake /N` : display commands without executing them
- `nmake /P` : display all macro definitions and target descriptions
- `nmake /K` : continue building unrelated targets on error
- `nmake /A` : build and ignore timestamp checks (force rebuild)
- `nmake /NOLOGO` : suppress copyright message

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
development, windows

## Links
- More information: https://learn.microsoft.com/cpp/build/reference/nmake-reference
- tldr page: https://tldr.inbrowser.app/pages/windows/nmake
