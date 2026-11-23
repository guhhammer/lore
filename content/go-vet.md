# go vet

Check Go source code and report suspicious constructs (e.g. lint your Go source files). Go vet returns a non-zero exit code if problems are found; returns a zero exit code if no problems are found.

## Usage
- `go vet` : check the Go package in the current directory
- `go vet path/to/file_or_directory` : check the Go package in the specified path
- `go tool vet help` : list available checks that can be run with go vet
- `go tool vet help check_name` : view details and flags for a particular check
- `go vet -c=n` : display offending lines plus `n` lines of surrounding context
- `go vet -json` : output analysis and errors in JSON format

## Categories
text, development, files

## Links
- More information: https://pkg.go.dev/cmd/vet
- tldr page: https://tldr.inbrowser.app/pages/common/go-vet
