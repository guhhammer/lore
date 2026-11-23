# ruff check

An extremely fast Python linter. `check` is the default command - it can be omitted everywhere. If no files or directories are specified, the current working directory is used by default.

## Usage
- `ruff check path/to/file_or_directory1 path/to/file_or_directory2 ...` : run the linter on the given files or directories
- `ruff check --fix` : apply the suggested fixes, modifying the files in-place
- `ruff check --watch` : run the linter and re-lint on change
- `ruff check --select ALL|rule_code1,rule_code2,...` : only enable the specified rules (or all rules), ignoring the configuration file
- `ruff check --extend-select rule_code1,rule_code2,...` : additionally enable the specified rules
- `ruff check --ignore rule_code1,rule_code2,...` : disable the specified rules
- `ruff check --select rule_code --add-noqa` : ignore all existing violations of a rule by adding `# noqa` directives to all lines that violate it

## Categories
files

## Links
- More information: https://docs.astral.sh/ruff/linter/
- tldr page: https://tldr.inbrowser.app/pages/common/ruff-check
