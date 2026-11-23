# godoc

View documentation for go packages.

## Usage
- `godoc fmt` : display help for a specific package
- `godoc fmt Printf` : display help for the function "Printf" of "fmt" package
- `godoc -http=:6060` : serve documentation as a web server on port 6060
- `godoc -write_index -index_files=path/to/file` : create an index file
- `godoc -http=:6060 -index -index_files=path/to/file` : use the given index file to search the docs

## Categories
files, network, packages

## Links
- More information: https://pkg.go.dev/golang.org/x/tools/cmd/godoc
- tldr page: https://tldr.inbrowser.app/pages/common/godoc
