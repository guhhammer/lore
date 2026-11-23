# go install

Compile and install packages named by the import paths.

## Usage
- `go install` : compile and install the current package
- `go install path/to/package` : compile and install a specific local package
- `go install golang.org/x/tools/gopls@latest` : install the latest version of a program, ignoring `go.mod` in the current directory
- `go install golang.org/x/tools/gopls` : install a program at the version selected by `go.mod` in the current directory

## Categories
packages, development, files

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Compile_and_install_packages_and_dependencies
- tldr page: https://tldr.inbrowser.app/pages/common/go-install
