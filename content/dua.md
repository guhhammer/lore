# dua

Dua (Disk Usage Analyzer): get the disk space usage of a directory.

## Usage
- `dua path/to/directory` : analyze specific directory
- `dua --apparent-size` : display apparent size instead of disk usage
- `dua --count-hard-links` : count hard-linked files each time they are seen
- `dua aggregate` : aggregate the consumed space of one or more directories or files
- `dua interactive` : launch the terminal user interface
- `dua --format metric|binary|bytes|GB|GiB|MB|MiB` : format printing byte counts
- `dua --threads count` : use a specific number of threads (defaults to the process number of threads)

## Categories
disks, files, monitoring

## Links
- More information: https://github.com/Byron/dua-cli
- tldr page: https://tldr.inbrowser.app/pages/common/dua
