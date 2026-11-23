# gofmt

Format Go source code.

## Usage
- `gofmt path/to/source.go` : format a file and display the result to the console
- `gofmt -w path/to/source.go` : format a file, overwriting the original file in-place
- `gofmt -s -w path/to/source.go` : format a file, and then simplify the code, overwriting the original file
- `gofmt -d path/to/source.go` : display diffs instead of rewriting files
- `gofmt -l path/to/directory` : list files that would be changed by gofmt
- `gofmt -r 'x == nil -> isNil(x)' path/to/source.go` : apply a rewrite rule to the source before reformatting (e.g. replace nil checks with `isNil` calls)
- `gofmt -e path/to/source.go` : report all errors, including multiple ones per line

## Categories
files, development

## Links
- More information: https://pkg.go.dev/cmd/gofmt
- tldr page: https://tldr.inbrowser.app/pages/common/gofmt
