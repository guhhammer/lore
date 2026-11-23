# gtar

This command is an alias of GNU `tar`.

## Usage
- `tar cf path/to/target.tar path/to/file1 path/to/file2 ...` : create an archive and write it to a file
- `tar czf path/to/target.tar.gz path/to/file1 path/to/file2 ...` : create a gzipped archive and write it to a file
- `tar czf path/to/target.tar.gz -C path/to/directory .` : create a gzipped (compressed) archive from a directory using relative paths
- `tar xvf path/to/source.tar.ext` : extract a (compressed) archive file into the current directory verbosely
- `tar xf path/to/source.tar.ext -C path/to/directory` : extract a (compressed) archive file into the target directory
- `tar caf path/to/target.tar.xz path/to/file1 path/to/file2 ...` : create a compressed archive and write it to a file, using the file extension to automatically determine the compression program
- `tar tvf path/to/source.tar` : list the contents of a tar file verbosely
- `tar xf path/to/source.tar --wildcards "*.html"` : extract files matching a pattern from an archive file

## Categories
archives, files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gtar
