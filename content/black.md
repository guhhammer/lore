# black

Format Python code automatically.

## Usage
- `black path/to/file_or_directory` : auto-format a file or entire directory
- `black -c "code"` : format the code passed in as a string
- `black --check path/to/file_or_directory` : show whether a file or a directory would have changes made to them if they were to be formatted
- `black --diff path/to/file_or_directory` : simulate formatting a file or a directory and show the changes that would be made
- `black -q path/to/file_or_directory` : auto-format a file or directory, emitting exclusively error messages to `stderr`
- `black -S path/to/file_or_directory` : auto-format a file or directory without replacing single quotes with double quotes (adoption helper, avoid using this for new projects)

## See also
- ruff

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/black
