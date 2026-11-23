# pip inspect

Inspect the Python environment and produce a report in JSON format.

## Usage
- `pip inspect` : inspect the current environment
- `pip inspect >environment_report.json` : inspect and save output to a file
- `pip inspect --local` : inspect only locally installed packages (not global)
- `pip inspect --user` : inspect only user-installed packages
- `pip inspect --path path/to/environment` : inspect packages in a specific path
- `pip inspect -v` : inspect with verbose output (Note: the `-v` flag can be repeated to increase verbosity)

## Categories
files, text, packages

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_inspect/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-inspect
