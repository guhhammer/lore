# go list

List packages or modules. Note: `./...` is a Go package pattern understood by Go tooling. It matches the current package and all packages recursively under the current directory.

## Usage
- `go list ./...` : list packages
- `go list std` : list standard packages
- `go list -json time net/http` : list packages in JSON format
- `go list -m -u all` : list module dependencies and available updates

## Categories
packages, text, files

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-List_packages_or_modules
- tldr page: https://tldr.inbrowser.app/pages/common/go-list
- Wikipedia: https://en.wikipedia.org/wiki/Go_(programming_language)
