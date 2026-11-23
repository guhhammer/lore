# kate

KDE's advanced text editor.

## Usage
- `kate path/to/file1 path/to/file2 ...` : open specific files
- `kate https://example.com/path/to/file1 https://example.com/path/to/file2 ...` : open specific remote files
- `kate -n` : create a new editor instance even if one is already open
- `kate -l line_number path/to/file` : open a file with the cursor at the specific line
- `kate -l line_number -c column_number path/to/file` : open a file with the cursor at the specific line and column
- `cat path/to/file | kate -i` : create a file from `stdin`
- `kate -h` : display help

## Categories
files, editors, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/kate
