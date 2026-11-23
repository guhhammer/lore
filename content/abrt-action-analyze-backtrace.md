# abrt-action-analyze-backtrace

Analyze C/C++ backtrace. Generate duplication hash, backtrace rating, and identify crash function. Save the data as new elements `duphash`, `rating`, `crash_function` in the problem directory.

## Usage
- `abrt-action-analyze-backtrace` : analyze backtrace for the current working directory
- `abrt-action-analyze-backtrace -d path/to/directory` : analyze backtrace for a specific directory
- `abrt-action-analyze-backtrace -v` : analyze backtrace verbosely

## Categories
files, security

## Links
- More information: https://manned.org/abrt-action-analyze-backtrace
- tldr page: https://tldr.inbrowser.app/pages/linux/abrt-action-analyze-backtrace
