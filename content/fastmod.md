# fastmod

A fast partial replacement for the `codemod` tool, replace and replace all in the whole codebase. Regexes are matched by Rust `regex` crate.

## Usage
- `fastmod regex replacement` : replace a `regex` in all files of the current directory, ignoring files on `.ignore` and `.gitignore`
- `fastmod --ignore-case regex replacement -- path/to/file_or_directory1 path/to/file_or_directory2 ...` : replace a `regex` in case-insensitive mode in specific files or directories
- `fastmod regex replacement --dir path/to/directory --iglob '**/*.{js,json}'` : replace a `regex` in a specific directory in files filtered with a case-insensitive `glob` pattern
- `fastmod --fixed-strings exact_string replacement --extensions json,js` : replace for an exact string in `.js` or JSON files
- `fastmod --accept-all --fixed-strings exact_string replacement` : replace for an exact string without prompt for a confirmation (disables `regex`)
- `fastmod --accept-all --print-changed-files --fixed-strings exact_string replacement` : replace for an exact string without prompt for a confirmation, printing changed files

## Categories
text, files

## Links
- More information: https://github.com/facebookincubator/fastmod
- tldr page: https://tldr.inbrowser.app/pages/common/fastmod
