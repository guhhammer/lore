# pacdiff

Maintenance utility for `.pacorig`, `.pacnew`, and `.pacsave` files created by `pacman`.

## Usage
- `pacdiff` : review files that need maintenance in interactive mode
- `pacdiff -s` : use `sudo` and `sudoedit` to remove and merge files
- `pacdiff -s -b` : review files needing maintenance, creating `.bak`ups of the original if you `(O)verwrite`
- `DIFFPROG=editor pacdiff` : use a specific editor to view and merge configuration files (default is `vim -d`)
- `pacdiff -l` : scan for configuration files with `locate` instead of using `pacman` database
- `pacdiff -h` : display help

## Categories
editors

## Links
- More information: https://manned.org/pacdiff
- tldr page: https://tldr.inbrowser.app/pages/linux/pacdiff
