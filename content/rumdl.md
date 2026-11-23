# rumdl

A fast Markdown linter and formatter written in Rust.

## Usage
- `rumdl check path/to/file_or_directory1 path/to/file_or_directory2 ...` : lint the specified files or directories
- `rumdl check -f path/to/file_or_directory` : lint and automatically fix issues where possible
- `rumdl check -d rule_id1,rule_id2,... path/to/file_or_directory` : lint with specific rules disabled
- `rumdl check -w path/to/file_or_directory` : re-run the linter whenever files change
- `rumdl fmt path/to/file_or_directory` : format files in place
- `rumdl init` : create a default configuration file (`.rumdl.toml`) in the current directory
- `rumdl rule MD013` : show information about a specific rule (list all rules if omitted)
- `rumdl version` : display version

## Categories
files, documents

## Links
- More information: https://rumdl.dev/usage/cli
- tldr page: https://tldr.inbrowser.app/pages/common/rumdl
