# pacini

Query pacman-style configuration files.

## Usage
- `pacini path/to/file` : show the full parsed configuration file (default: `stdin`)
- `pacini path/to/file --section-list` : list configured sections
- `pacini path/to/file --verbose` : always show directive names even if only one directive is provided
- `pacini path/to/file --section section_name` : display directives listed in a specific section
- `pacini --help` : display help
- `pacini --version` : display version

## Categories
files, databases, desktop

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/pacini.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/pacini
