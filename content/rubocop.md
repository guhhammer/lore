# rubocop

Lint Ruby files.

## Usage
- `rubocop` : check all files in the current directory (including subdirectories)
- `rubocop path/to/file_or_directory1 path/to/file_or_directory2 ...` : check one or more specific files or directories
- `rubocop --out path/to/file` : write output to file
- `rubocop --show-cops` : view list of cops (linter rules)
- `rubocop --except cop1 cop2 ...` : exclude a cop
- `rubocop --only cop1 cop2 ...` : run only specified cops
- `rubocop --auto-correct` : auto-correct files (experimental)

## Categories
files

## Links
- More information: https://docs.rubocop.org/rubocop/latest/usage/cli_reference.html
- tldr page: https://tldr.inbrowser.app/pages/common/rubocop
