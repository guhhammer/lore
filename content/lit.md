# lit

LLVM integrated tester for executing LLVM and Clang style test suites, summarizing results. Part of LLVM.

## Usage
- `lit path/to/test_file.test` : run a specified test case
- `lit path/to/test_suite` : run all test cases in a specified directory
- `lit path/to/test_suite --time-tests` : run all test cases and check the wall time for each cases, then report to summary output
- `lit path/to/test_file.test --vg --vg-leak --vg-args=args_to_valgrind` : run individual tests with Valgrind (memory check and memory leak test)

## Categories
development, files

## Links
- More information: https://www.llvm.org/docs/CommandGuide/lit.html
- tldr page: https://tldr.inbrowser.app/pages/common/lit
