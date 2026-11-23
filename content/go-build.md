# go build

Compile Go sources.

## Usage
- `go build path/to/main.go` : compile a 'package main' file (output will be the filename without extension)
- `go build -o path/to/binary path/to/source.go` : compile, specifying the output filename
- `go build -o path/to/binary path/to/package` : compile a package
- `go build -race -o path/to/executable path/to/main_package` : compile a main package into an executable, enabling data race detection

## Categories
development, files, packages

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Compile_packages_and_dependencies
- tldr page: https://tldr.inbrowser.app/pages/common/go-build
