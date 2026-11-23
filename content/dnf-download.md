# dnf download

Download RPM packages from the DNF repositories. Not default to `dnf` but supported via `dnf-plugins-core`.

## Usage
- `dnf download package` : download the latest version of a package to the current directory
- `dnf download package --destdir path/to/directory` : download a package to a specific directory (the directory must exist)
- `dnf download --url package` : print the URL where the RPM package can be downloaded from

## Categories
network, packages, files

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/download.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-download
