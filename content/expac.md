# expac

A data extraction tool for alpm databases, offering printf-like flexibility for pacman-based utilities.

## Usage
- `expac -S '%D' package` : list the dependencies of a package
- `expac -S "%o" package` : list the optional dependencies of a package
- `expac -S -H M '%k\t%n' package1 package2 ...` : list the download size of packages in MiB
- `expac -S -H M '%k\t%n' $(pacman -Qqu) | sort -sh` : list packages marked for upgrade with their download size
- `expac -d '\n\n' -l '\n\t' -Q '%n\n\t%O' $(pacman -Qeq)` : list explicitly-installed packages with their optional dependencies

## See also
- pacman

## Categories
packages, network

## Links
- More information: https://github.com/falconindy/expac#name
- tldr page: https://tldr.inbrowser.app/pages/linux/expac
