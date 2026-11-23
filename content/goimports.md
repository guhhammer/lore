# goimports

Update Go import lines, adding missing ones and removing unreferenced ones.

## Usage
- `goimports path/to/file.go` : display the completed import source file
- `goimports -w path/to/file.go` : write the result back to the source file instead of `stdout`
- `goimports -w -d path/to/file.go` : display diffs and write the result back to the source file
- `goimports -local path/to/package1,path/to/package2,... path/to/file.go` : set the import prefix string after 3rd-party packages (comma-separated list)

## Categories
files, text

## Links
- More information: https://pkg.go.dev/golang.org/x/tools/cmd/goimports
- tldr page: https://tldr.inbrowser.app/pages/common/goimports
