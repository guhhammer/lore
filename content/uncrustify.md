# uncrustify

C, C++, C#, D, Java, and Pawn source code formatter.

## Usage
- `uncrustify -f path/to/file.cpp -o path/to/output.cpp` : format a single file
- `find . -name "*.cpp" | uncrustify -F - --replace` : read filenames from `stdin`, and take backups before writing output back to the original filepaths
- `find . -name "*.cpp" | uncrustify -F - --no-backup` : don't make backups (useful if files are under version control)
- `uncrustify -c path/to/uncrustify.cfg -f path/to/file.cpp` : use a custom configuration file and write the result to `stdout`
- `uncrustify --set option=value` : explicitly set a configuration variable's value
- `uncrustify --update-config -o path/to/new.cfg` : generate a new configuration file

## Categories
files, development

## Links
- More information: https://manned.org/uncrustify
- tldr page: https://tldr.inbrowser.app/pages/common/uncrustify
