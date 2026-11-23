# vitest

Fast, modern testing framework built for Vite, offering seamless integration, TypeScript support, and a Jest-compatible API for unit, integration, and snapshot testing.

## Usage
- `vitest run` : run all available tests
- `vitest run path/to/file1 path/to/file2 ...` : run the test suites from the given files
- `vitest run regex1 regex2 ...` : run the test suites from files within the current and subdirectories, whose paths match the given `regex`
- `vitest run -t regex` : run the tests whose names match the given `regex`
- `vitest` : watch files for changes and automatically re-run related tests
- `vitest run --coverage` : run tests with coverage
- `vitest run --bail=1` : run all tests but stops immediately after the first test failure
- `vitest --help` : display help

## Categories
development, text

## Links
- More information: https://vitest.dev/guide/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/vitest
