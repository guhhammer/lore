# xbps-query

XBPS utility to query for package and repository information.

## Usage
- `xbps-query -s regex|keyword --repository --regex` : search for a package in remote repositories using a `regex` or a keyword (if `--regex` is omitted)
- `xbps-query -S package` : show information about an installed package
- `xbps-query -S package --repository` : show information about a package in remote repositories
- `xbps-query -l` : list packages registered in the package database
- `xbps-query -m` : list explicitly installed packages (i.e. not automatically installed as dependencies)

## See also
- xbps

## Categories
packages, databases, text

## Links
- More information: https://manned.org/xbps-query
- tldr page: https://tldr.inbrowser.app/pages/linux/xbps-query
