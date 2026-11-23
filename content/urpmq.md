# urpmq

Query information about packages and media in Mageia.

## Usage
- `urpmq -i package` : display information about an installable package
- `urpmq --requires package` : display direct dependencies of a package
- `urpmq -d package` : display direct and indirect dependencies of a package
- `sudo urpmq -d -m --sources path/to/file.rpm` : list the not installed packages needed for an RPM file with their sources
- `urpmq --list-media --list-url` : list all configured media with their URLs, including inactive media
- `urpmq -g -r -y keyword` : search for a package printing group, version, and release
- `urpmq -g -r package` : search for a package with using its exact name

## See also
- urpmi
- urpmi.update
- urpmi.addmedia
- urpmi.removemedia
- urpmf
- urpme

## Categories
packages, databases, desktop

## Links
- More information: https://man.linuxreviews.org/man8/urpmq.8.html
- tldr page: https://tldr.inbrowser.app/pages/linux/urpmq
