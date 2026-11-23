# go test

Test Go packages (files have to end with `_test.go`). Note: `./...` is a Go package pattern understood by Go tooling. It matches the current package and all packages recursively under the current directory.

## Usage
- `go test` : test the package found in the current directory
- `go test -v` : verbosely test the package in the current directory
- `go test -v ./...` : test the packages in the current directory and all subdirectories
- `go test -v -bench .` : test the package in the current directory and run all benchmarks
- `go test -v -bench . -benchtime 50s` : test the package in the current directory and run all benchmarks for 50 seconds
- `go test -cover` : test the package with coverage analysis

## Categories
development, packages, files

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Testing_flags
- tldr page: https://tldr.inbrowser.app/pages/common/go-test
