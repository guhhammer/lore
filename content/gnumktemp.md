# gnumktemp

This command is an alias of GNU `mktemp`.

## Usage
- `mktemp` : create an empty temporary file and print its absolute path
- `mktemp -p /path/to/temporary_directory` : use a custom directory if `$TMPDIR` is not set (the default is platform-dependent, but usually `/tmp`)
- `mktemp /tmp/example.XXXXXXXX` : use a custom path template (`X`s are replaced with random alphanumeric characters)
- `mktemp -t example.XXXXXXXX` : use a custom file name template
- `mktemp -d` : create an empty temporary directory and print its absolute path

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnumktemp
