# gopls

The official Go language server.

## Usage
- `gopls serve` : start the language server
- `gopls check path/to/file1.go path/to/file2.go ...` : show diagnostics for one or more files
- `gopls format -w path/to/file1.go path/to/file2.go ...` : format one or more files in-place
- `gopls imports -w path/to/file.go` : add missing and remove unused imports in a file in-place
- `gopls definition path/to/file.go:line:column` : show the definition of the identifier at a specific line and column
- `gopls references path/to/file.go:line:column` : list all references to the identifier at a specific line and column
- `gopls rename -w path/to/file.go:line:column new_name` : rename the identifier at a specific line and column, writing the changes to the affected files
- `gopls version` : display version

## Categories
files, text

## Links
- More information: https://go.dev/gopls/
- tldr page: https://tldr.inbrowser.app/pages/common/gopls
