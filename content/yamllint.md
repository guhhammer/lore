# yamllint

A linter for YAML files.

## Usage
- `yamllint path/to/file.yaml` : lint a file
- `yamllint path/to/directory` : lint all YAML files in a directory recursively
- `yamllint -c path/to/config.yaml path/to/file.yaml` : lint a file with a specific config file
- `yamllint -d "extends: relaxed" path/to/file.yaml` : lint a file using a preset config (`default` or `relaxed`)
- `yamllint -f parsable path/to/file.yaml` : output in a parsable format (for CI integration)
- `yamllint -s path/to/file.yaml` : lint a file with strict mode (return 2 for warnings)
- `cat path/to/file.yaml | yamllint -` : read from `stdin`
- `yamllint -h` : display help

## Categories
files, text

## Links
- More information: https://yamllint.readthedocs.io
- tldr page: https://tldr.inbrowser.app/pages/common/yamllint
