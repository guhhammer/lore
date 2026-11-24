# cmake

Cross-platform build automation system, that generates recipes for native build systems.

## Usage
- `cmake path/to/project_directory` : generate a build recipe in the current directory with `CMakeLists.txt` from a project directory
- `cmake --build path/to/build_directory` : use a generated recipe in a given directory to build artifacts
- `cmake --install path/to/build_directory --strip` : install the build artifacts into `/usr/local/` and strip debugging symbols
- `cmake path/to/project_directory -D CMAKE_BUILD_TYPE=Release` : generate a build recipe, with build type set to `Release` with CMake variable
- `cmake -G generator_name path/to/project_directory` : generate a build recipe using `generator_name` as the underlying build system
- `cmake --install path/to/build_directory --strip --prefix path/to/directory` : install the build artifacts using a custom prefix for paths
- `cmake --build path/to/build_directory -t target_name` : run a custom build target
- `cmake -h` : display help

## Categories
development, files, packages

## Links
- More information: https://cmake.org/cmake/help/latest/manual/cmake.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/cmake
- Wikipedia: https://en.wikipedia.org/wiki/CMake
