# stat

Display file and filesystem information.

## Usage
- `stat path/to/file` : display properties about a specific file such as size, permissions, creation date, and access date among others
- `stat -t path/to/file` : display properties about a specific file, only showing the raw result data without labels
- `stat -f path/to/file` : display information about the filesystem where a specific file is located
- `stat -c "%a %n" path/to/file` : show only octal file permissions
- `stat -c "%U %G" path/to/file` : show the owner and group of a specific file
- `stat -c "%s %n" path/to/file` : show the size of a specific file in bytes

## See also
- file

## Categories
files, shell

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stat-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/stat
- Wikipedia: https://en.wikipedia.org/wiki/Stat_(system_call)
