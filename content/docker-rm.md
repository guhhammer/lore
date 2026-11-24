# docker rm

This command is an alias of `docker container rm`.

## Usage
- `rm path/to/file1 path/to/file2 ...` : remove specific files
- `rm -f path/to/file1 path/to/file2 ...` : remove specific files ignoring nonexistent ones
- `rm -i path/to/file1 path/to/file2 ...` : remove specific files interactively prompting before each removal
- `rm -v path/to/file1 path/to/file2 ...` : remove specific files printing info about each removal
- `rm -r path/to/file_or_directory1 path/to/file_or_directory2 ...` : remove specific files and directories recursively
- `rm -d path/to/directory` : remove empty directories (this is considered the safe method)

## Categories
files, containers, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/docker-rm
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
