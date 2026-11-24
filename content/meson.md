# meson

SCons-like build system that uses Python as a front-end language and Ninja as a building backend.

## Usage
- `meson init` : generate a project with default values
- `meson init -l c -n myproject --version 0.1` : generate a C project with a given name and version
- `meson build` : configure a directory named `build` with default values for compilation
- `meson compile -C path/to/build_directory` : compile the project
- `meson test -C path/to/build_directory` : run all tests in the project
- `meson install -C path/to/build_directory` : install the project to `/usr/local`
- `meson -h` : display help
- `meson -v` : display version

## Categories
development, files

## Links
- More information: https://mesonbuild.com/Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/meson
- Wikipedia: https://en.wikipedia.org/wiki/Meson_(software)
