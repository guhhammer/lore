# vale

Extensible style checker that supports multiple markup formats, such as Markdown and AsciiDoc.

## Usage
- `vale path/to/file` : check the style of a file
- `vale --config='path/to/.vale.ini' path/to/file` : check the style of a file with a specified configuration
- `vale --output=JSON path/to/file` : output the results in JSON format
- `vale --minAlertLevel=suggestion|warning|error path/to/file` : check style issues at the specific severity and higher
- `cat file.md | vale --ext=.md` : check the style from `stdin`, specifying markup format
- `vale ls-config` : list the current configuration

## Categories
files, documents

## Links
- More information: https://vale.sh/docs/cli
- tldr page: https://tldr.inbrowser.app/pages/common/vale
