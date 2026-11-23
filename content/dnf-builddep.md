# dnf builddep

Install dependencies to build a given package. Not default to `dnf` but supported via `dnf-plugins-core`.

## Usage
- `dnf builddep path/to/specification.spec` : install dependencies for a given package
- `dnf builddep --skip-unavailable path/to/specification.spec` : install dependencies for a given package but ignore unavailable
- `dnf builddep -D 'expression'` : define the RPM macro to a given expression
- `dnf builddep --spec argument` : define an argument for a `.spec` file path
- `dnf builddep --srpm argument` : define an argument for a `.rpm` file path
- `dnf builddep --help-cmd` : display help

## Categories
packages, files, development

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/builddep.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-builddep
