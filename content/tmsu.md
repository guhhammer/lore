# tmsu

Simple tool for tagging files.

## Usage
- `tmsu tag path/to/file.mp3 music big-jazz mp3` : tag a specific file with multiple tags
- `tmsu tag --tags "music mp3" *.mp3` : tag multiple files
- `tmsu tags *.mp3` : list tags of specified file(s)
- `tmsu files big-jazz music` : list files with specified tag(s)
- `tmsu files "(year >= 1990 and year <= 2000) and grunge"` : list files with tags matching boolean expression
- `tmsu mount path/to/directory` : mount tmsu virtual filesystem to an existing directory

## Categories
files, disks, media

## Links
- More information: https://tmsu.org/
- tldr page: https://tldr.inbrowser.app/pages/common/tmsu
