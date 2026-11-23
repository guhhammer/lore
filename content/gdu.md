# gdu

Disk usage analyzer with console interface.

## Usage
- `gdu` : interactively show the disk usage of the current directory
- `gdu path/to/directory` : interactively show the disk usage of a given directory
- `gdu --show-disks` : interactively show the disk usage of all mounted disks
- `gdu --ignore-dirs path/to/directory1,path/to/directory2,...` : interactively show the disk usage of the current directory but ignore some sub-directories
- `gdu --ignore-dirs-pattern '.*[abc]+'` : ignore paths by `regex`
- `gdu --no-hidden` : ignore hidden directories
- `gdu --non-interactive path/to/directory` : only print the result, do not enter interactive mode
- `gdu --no-progress path/to/directory` : do not show the progress in non-interactive mode (useful in scripts)

## Categories
files, disks, monitoring

## Links
- More information: https://github.com/dundee/gdu#usage
- tldr page: https://tldr.inbrowser.app/pages/common/gdu
