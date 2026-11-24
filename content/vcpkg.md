# vcpkg

Package manager for C/C++ libraries. Note: Packages are not installed in the system. To use them, you need to tell your build system (e.g. CMake) to use `vckg`.

## Usage
- `vcpkg install curl` : build and add package `libcurl` to the `vcpkg` environment
- `vcpkg install --triplet=wasm32-emscripten zlib` : build and add `zlib` using the `emscripten` toolchain
- `vcpkg search pkg_name` : search for a package
- `cmake -B build -DCMAKE_TOOLCHAIN_FILE=path/to/vcpkg_install_directory/scripts/buildsystems/vcpkg.cmake` : configure a CMake project to use `vcpkg` packages

## Categories
packages, development

## Links
- More information: https://learn.microsoft.com/vcpkg/
- tldr page: https://tldr.inbrowser.app/pages/common/vcpkg
- Wikipedia: https://en.wikipedia.org/wiki/Vcpkg
