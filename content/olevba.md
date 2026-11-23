# olevba

Parse OLE and OpenXML files (e.g., DOC, XLS, PPT, etc.) to extract VBA macros, deobfuscate, and analyze malicious code. Part of the `python-oletools` suite.

## Usage
- `olevba path/to/file` : analyze a file, showing both macro code and analysis results
- `olevba -r path/to/directory` : recursively analyze all supported files in a directory
- `olevba -p password path/to/encrypted_file` : provide a password for encrypted Microsoft Office files (may be repeated)
- `olevba -a path/to/file` : display only analysis results, without showing macro source code
- `olevba -c path/to/file` : display only macro source code
- `olevba --decode path/to/file` : show obfuscated strings and their decoded content

## Categories
files, archives

## Links
- More information: https://github.com/decalage2/oletools
- tldr page: https://tldr.inbrowser.app/pages/common/olevba
