# make

Task runner for targets described in Makefile. Mostly used to control the compilation of an executable from source code.

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
development

## Links
- More information: https://www.gnu.org/software/make/manual/make.html
- tldr page: https://tldr.inbrowser.app/pages/common/make
