# biome

A toolchain for formatting, linting, and checking code quality for web projects.

## Usage
- `biome init` : initialize a new Biome configuration file in the current directory
- `biome format --write path/to/file_or_directory` : format files and overwrite them in place
- `biome format path/to/file_or_directory` : check formatting without applying changes
- `biome lint --write path/to/file_or_directory` : lint files and apply safe fixes
- `biome check --write path/to/file_or_directory` : run format, lint, and import sorting, applying safe fixes
- `biome ci path/to/file_or_directory` : run all checks in CI mode (no fixes applied, exits with error if issues found)
- `biome explain rule_name` : explain a specific lint rule
- `biome -V` : display version

## Categories
files

## Links
- More information: https://biomejs.dev/reference/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/biome
