# zpaq

Incremental journaling backup utility and archiver.

## Usage
- `zpaq a path/to/archive.zpaq path/to/file_or_directory` : add a file or directory to a new or existing archive
- `zpaq a -kpassword path/to/archive.zpaq path/to/file_or_directory` : create or add to an encrypted archive
- `zpaq x path/to/archive.zpaq` : extract the most recent versions of files
- `zpaq l path/to/archive.zpaq` : list the archive contents
- `zpaq a path/to/archive.zpaq -m1..5 path/to/file_or_directory` : set the level of compression (higher means more compression but slower)
- `zpaq x path/to/archive.zpaq path/in/archive/to/extract -to path/to/output -until YYYY-MM-DD` : extract the specified files from the archive that are not newer than the specified date

## Categories
archives, files

## Links
- More information: https://mattmahoney.net/dc/zpaqdoc.html
- tldr page: https://tldr.inbrowser.app/pages/common/zpaq
