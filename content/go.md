# go

Manage Go source code. Some subcommands such as `build` have their own usage documentation.

## Usage
- `go get path/to/package` : download and install a package, specified by its import path
- `go run file.go` : compile and run a source file (it has to contain a `main` package)
- `go build -o executable file.go` : compile a source file into a named executable
- `go build` : compile the package present in the current directory
- `go test` : execute all test cases of the current package (files have to end with `_test.go`)
- `go install` : compile and install the current package
- `go mod init module_name` : initialize a new module in the current directory

## Categories
development, packages, files

## Links
- More information: https://pkg.go.dev/cmd/go
- tldr page: https://tldr.inbrowser.app/pages/common/go
- Wikipedia: https://en.wikipedia.org/wiki/Go_(programming_language)
