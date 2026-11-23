# scalafmt

Code formatter for Scala. Configurations are stored in the `.scalafmt.conf` file.

## Usage
- `scalafmt` : reformat all `.scala` files in the current directory recursively
- `scalafmt --config path/to/.scalafmt.conf path/to/file_or_directory1 path/to/file_or_directory2 ...` : reformat specific files or directories with a custom formatting configuration
- `scalafmt --config path/to/.scalafmt.conf --test` : check if files are correctly formatted, returning `0` if all files respect the formatting style
- `scalafmt --exclude path/to/file_or_directory1 path/to/file_or_directory2 ...` : exclude files or directories
- `scalafmt --config path/to/.scalafmt.conf --mode diff` : format only files that were edited against the current Git branch

## Categories
files, version-control

## Links
- More information: https://scalameta.org/scalafmt/
- tldr page: https://tldr.inbrowser.app/pages/common/scalafmt
