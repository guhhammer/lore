# rnm

Bulk Rename Utility.

## Usage
- `rnm -ss old -rs new path/to/directory` : replace a search string with a replacement string in filenames
- `rnm -ssf old -rs new path/to/files` : use a fixed (literal) search and replace string instead of `regex`
- `rnm -i 1 -inc 1 -rs _ path/to/files` : add an auto-incremented index to filenames starting from 1
- `rnm -ns/f path/to/names.txt path/to/files` : rename files using a list of new names from a text file
- `rnm -fo -ss pattern -rs replacement path/to/files` : rename only files (ignoring directories and links)
- `rnm -s/mt -ss pattern -rs replacement path/to/files` : sort input files by modification time before renaming
- `rnm -sim -ss pattern -rs replacement path/to/files` : run a simulation without making actual changes
- `rnm -u` : undo the last renaming operation

## Categories
files, text

## Links
- More information: https://github.com/neurobin/rnm#basic-options
- tldr page: https://tldr.inbrowser.app/pages/linux/rnm
