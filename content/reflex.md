# reflex

Watch a directory and rerun a command when certain files change.

## Usage
- `reflex make` : rebuild with `make` if any file changes
- `reflex -r '\.go$' go run .` : compile and run Go application if any `.go` file changes
- `reflex -G '^directory_name/' command` : ignore a directory when watching for changes
- `reflex -s true command` : run command when reflex starts and restarts on file changes
- `reflex -- echo {}` : substitute the filename that changed in

## Categories
files

## Links
- More information: https://github.com/cespare/reflex#usage
- tldr page: https://tldr.inbrowser.app/pages/common/reflex
