# mado

A fast Markdown linter for CommonMark and GitHub Flavored Markdown.

## Usage
- `mado check` : check all Markdown files in the current directory recursively
- `mado check path/to/file_or_directory1 path/to/file_or_directory2 ...` : check specific files or directories
- `mado check --exclude "**/node_modules/**" path/to/directory` : exclude files matching a `glob` pattern
- `mado --config path/to/mado.toml check path/to/directory` : check using a specific configuration file
- `mado check path/to/directory --output-format concise|mdl|markdownlint` : display violations in a specific format
- `mado generate-shell-completion bash|elvish|fish|powershell|zsh` : generate a completion script for a specific shell
- `mado --help` : display help
- `mado --version` : display version

## Categories
files, documents, desktop

## Links
- More information: https://github.com/akiomik/mado
- tldr page: https://tldr.inbrowser.app/pages/common/mado
