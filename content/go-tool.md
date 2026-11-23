# go tool

Run a Go tool or command. Execute a Go command as a stand-alone binary, typically for debugging.

## Usage
- `go tool` : list available tools
- `go tool link path/to/main.o` : run the go link tool
- `go tool -n command arguments` : print the command that would be executed, but do not execute it (similar to `whereis`)
- `go tool command --help` : view documentation for a specified tool
- `go tool dist list` : list all available cross-compilation targets

## Categories
files

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Run_specified_go_tool
- tldr page: https://tldr.inbrowser.app/pages/common/go-tool
