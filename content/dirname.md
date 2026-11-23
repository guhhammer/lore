# dirname

Remove trailing filename portion from a path.

## Usage
- `dirname path/to/file_or_directory` : calculate the parent directory of a given path
- `dirname path/to/file_or_directory1 path/to/file_or_directory2 ...` : calculate the parent directory of multiple paths
- `dirname -z path/to/file_or_directory1 path/to/file_or_directory2 ...` : delimit output with a NUL character instead of a newline (useful when combining with `xargs`)

## See also
- basename

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/dirname
