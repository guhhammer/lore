# strip-nondeterminism

Remove non-deterministic information (e.g. timestamps) from files.

## Usage
- `strip-nondeterminism path/to/file` : strip nondeterministic information from a file
- `strip-nondeterminism --type filetype path/to/file` : strip nondeterministic information from a file manually specifying the filetype
- `strip-nondeterminism --timestamp unix_timestamp path/to/file` : strip nondeterministic information from a file; instead of removing timestamps set them to the specified UNIX timestamp

## Categories
files

## Links
- More information: https://salsa.debian.org/reproducible-builds/strip-nondeterminism
- tldr page: https://tldr.inbrowser.app/pages/common/strip-nondeterminism
