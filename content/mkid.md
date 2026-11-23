# mkid

Build an ID database for use with `lid` and other idutils tools.

## Usage
- `mkid` : build an ID database for the current directory
- `mkid path/to/directory1 path/to/directory2 ...` : build an ID database for specific directories
- `mkid -o path/to/database.id` : build an ID database and save it to a specific file
- `mkid -i "language1 language2 ..."` : include only specific languages
- `mkid -p path/to/excluded_directory` : exclude specific directories from indexing
- `mkid -s` : display statistics after building the database

## Categories
databases, files, development

## Links
- More information: https://www.gnu.org/software/idutils/manual/idutils.html#mkid-invocation
- tldr page: https://tldr.inbrowser.app/pages/linux/mkid
