# go get

Add a dependency package, or download packages in legacy GOPATH mode. Note: `./...` is a Go package pattern understood by Go tooling. It matches the current package and all packages recursively under the current directory.

## Usage
- `go get example.com/pkg` : add a specified package to `go.mod` in module-mode or install the package in GOPATH-mode
- `go get example.com/pkg@v1.2.3` : modify the package with a given version in module-aware mode
- `go get example.com/pkg@none` : remove a specified package
- `go get -u example.com/pkg` : update a package to its latest release
- `go get -u` : update direct dependencies
- `go get -u ./...` : update direct and indirect dependencies
- `go get -u -t ./...` : update direct, indirect, and test dependencies

## Categories
packages, network, development

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/go-get
- Wikipedia: https://en.wikipedia.org/wiki/Go_(programming_language)
