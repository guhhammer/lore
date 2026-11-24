# go fmt

Format Go source files, printing the changed filenames. Note: `./...` is a Go package pattern understood by Go tooling. It matches the current package and all packages recursively under the current directory.

## Usage
- `go fmt` : format Go source files in the current directory
- `go fmt path/to/package` : format a specific Go package in your import path (`$GOPATH/src`)
- `go fmt ./...` : format the package in the current directory and all subdirectories (note the `...`)
- `go fmt -n` : print what format commands would've been run, without modifying anything
- `go fmt -x` : print which format commands are run as they are run

## Categories
packages, files, text

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Gofmt__reformat__package_sources
- tldr page: https://tldr.inbrowser.app/pages/common/go-fmt
- Wikipedia: https://en.wikipedia.org/wiki/Go_(programming_language)
