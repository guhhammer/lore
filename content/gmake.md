# gmake

This command is an alias of GNU `make`.

## Usage
- `make` : call the first target specified in the Makefile (usually named "all")
- `make install|check|clean|uninstall|...` : call a specific target
- `make -j 4 target` : call a specific target, executing 4 jobs at a time in parallel
- `make -f path/to/file` : use a specific Makefile
- `make -C path/to/directory` : execute make from another directory
- `make -B target` : force making of a target, even if source files are unchanged
- `make target variable=new_value` : override a variable defined in the Makefile
- `make -e target` : override variables defined in the Makefile by the environment

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gmake
