# xgettext

Extract gettext strings from code files.

## Usage
- `xgettext path/to/input_file` : scan file and output strings to `messages.po`
- `xgettext -o path/to/output_file path/to/input_file` : use a different output filename
- `xgettext -j -o path/to/output_file path/to/input_file` : append new strings to an existing file
- `xgettext --omit-header path/to/input_file` : don't add a header containing metadata to the output file
- `xgettext -h` : display help

## Categories
files, archives

## Links
- More information: https://www.gnu.org/software/gettext/manual/gettext.html#xgettext-Invocation
- tldr page: https://tldr.inbrowser.app/pages/common/xgettext
