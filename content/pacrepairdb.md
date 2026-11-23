# pacrepairdb

Fix corrupted database entries in libalpm database.

## Usage
- `pacrepairdb package_name` : perform a basic repair on a specific package
- `pacrepairdb package_name --dbonly` : update the database entries without extracting or removing any packages
- `pacrepairdb package_name --print-only` : display the packages to be repaired and the cache packages to be used without making any changes
- `pacrepairdb package_name --verbose` : display additional progress and debug information
- `pacrepairdb --help` : display help
- `pacrepairdb --version` : display version

## Categories
databases, desktop, packages

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/pacrepairdb.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/pacrepairdb
