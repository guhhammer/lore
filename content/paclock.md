# paclock

Lock/unlock the libalpm database (used by `pacman`) to prevent or allow simultaneous package management operations.

## Usage
- `sudo paclock` : lock the database
- `paclock --print` : write the lock file path to `stdout` (without locking the database)
- `sudo paclock --unlock` : unlock the database
- `paclock --help` : display help
- `paclock --version` : display version

## Categories
databases, packages

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/paclock.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/paclock
