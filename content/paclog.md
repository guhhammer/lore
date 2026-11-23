# paclog

Filter pacman log entries.

## Usage
- `paclog` : display the entire pacman log
- `paclog --commandline` : display pacman-style logged commandline entries
- `paclog --package package_name` : display log events for a specific package
- `paclog --action install|reinstall|upgrade|downgrade|remove|all` : display package actions of a specific type
- `paclog --warnings` : display only errors, warnings, and notes
- `paclog --pkglist` : display the list of installed packages according to the log
- `paclog --help` : display help
- `paclog --version` : display version

## Categories
desktop, monitoring, packages

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/paclog.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/paclog
