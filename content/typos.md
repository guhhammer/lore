# typos

Find and correct spelling mistakes in source code.

## Usage
- `typos path/to/file_or_directory` : check a file or directory for spelling mistakes
- `typos -w path/to/file_or_directory` : fix typos automatically in a file or directory
- `typos --diff path/to/file_or_directory` : preview the changes before applying them
- `typos --hidden path/to/file_or_directory` : check a file or directory, including hidden files and directories
- `typos --exclude pattern path/to/file_or_directory` : check a file or directory, ignoring files that match a specific `glob` pattern
- `typos --dump-config path/to/typos.toml` : write the current configuration to a specific file
- `typos --type-list` : list all supported file types
- `typos --format brief|long|json|... path/to/file_or_directory` : show spelling mistakes in a specific output format (defaults to `long`)

## Categories
files, development, text

## Links
- More information: https://github.com/crate-ci/typos/blob/master/docs/reference.md
- tldr page: https://tldr.inbrowser.app/pages/common/typos
