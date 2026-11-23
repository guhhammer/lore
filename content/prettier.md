# prettier

An opinionated code formatter for JavaScript, JSON, CSS, YAML, and more.

## Usage
- `prettier path/to/file` : format a file and print the result to `stdout`
- `prettier --check path/to/file` : check if a specific file has been formatted
- `prettier --config path/to/config_file path/to/file` : run with a specific configuration file
- `prettier --write path/to/file_or_directory` : format a file or directory, replacing the original
- `prettier --single-quote --trailing-comma none --write path/to/file_or_directory` : format files or directories recursively using single quotes and no trailing commas
- `prettier --write "**/*.{js,jsx,ts,tsx}"` : format JavaScript and TypeScript files recursively, replacing the original

## Categories
files, text

## Links
- More information: https://prettier.io/docs/cli
- tldr page: https://tldr.inbrowser.app/pages/common/prettier
