# ast-grep

Search, lint, and rewrite code using AST patterns.

## Usage
- `ast-grep run -p 'pattern' path/to/file` : search for a pattern in files
- `ast-grep run -p 'pattern' -l python path/to/directory` : search for a pattern in a specific language
- `ast-grep run -p 'pattern' -r 'replacement' path/to/file` : rewrite code matching a pattern
- `ast-grep scan -r path/to/rule.yml path/to/directory` : run rules from a configuration file
- `ast-grep run -p 'pattern' -i path/to/directory` : interactively search and rewrite code
- `ast-grep run -h` : display help for a subcommand

## Categories
files, text

## Links
- More information: https://ast-grep.github.io/reference/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/ast-grep
