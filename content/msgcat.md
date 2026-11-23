# msgcat

Concatenate and merge multiple `.po` translation files. Useful in software localization pipelines to combine message catalogs with filtering options.

## Usage
- `msgcat file1.po file2.po ... -o combined.po` : combine multiple `.po` files into one
- `msgcat -f path/to/file_list.txt -o combined.po` : combine input files listed in a text file
- `msgcat -t UTF-8 input.po -o output.po` : set the output encoding (e.g. UTF-8)
- `msgcat -u file1.po file2.po ... -o unique.po` : output only unique messages (appearing in one file only)
- `msgcat --use-first file1.po file2.po ... -o output.po` : use the first available translation for duplicate entries
- `msgcat -h` : display help

## Categories
files, version-control

## Links
- More information: https://www.gnu.org/software/gettext/manual/gettext.html#msgcat-Invocation
- tldr page: https://tldr.inbrowser.app/pages/common/msgcat
