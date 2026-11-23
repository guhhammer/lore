# jest

A zero-configuration JavaScript testing platform.

## Usage
- `jest` : run all available tests
- `jest path/to/file1 path/to/file2 ...` : run the test suites from the given files
- `jest regex1 regex2 ...` : run the test suites from files within the current and subdirectories, whose paths match the given `regex`
- `jest --testNamePattern regex` : run the tests whose names match the given `regex`
- `jest --findRelatedTests path/to/source_file.js` : run test suites related to a given source file
- `jest --onlyChanged` : run test suites related to all uncommitted files
- `jest --watch` : watch files for changes and automatically re-run related tests
- `jest --help` : display help

## Categories
development, files, text

## Links
- More information: https://jestjs.io/docs/cli
- tldr page: https://tldr.inbrowser.app/pages/common/jest
