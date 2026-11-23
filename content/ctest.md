# ctest

CMake test driver program.

## Usage
- `ctest` : run all tests in the current CMake build directory
- `ctest -j 4` : run all tests defined in the CMake build directory, executing 4 jobs at a time in parallel
- `ctest -j` : run all tests on all processor cores
- `ctest --output-on-failure` : run all tests defined in the CMake build directory and print detailed logs on failed tests
- `ctest -N` : list available tests
- `ctest --output-on-failure -R '^test_name$'` : run a single test based on its name, or filter on a `regex`

## Categories
development, files

## Links
- More information: https://manned.org/ctest
- tldr page: https://tldr.inbrowser.app/pages/common/ctest
