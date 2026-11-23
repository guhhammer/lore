# govulncheck

Report known vulnerabilities that affect Go code. Note: `./...` is a Go package pattern understood by Go tooling. It matches the current package and all packages recursively under the current directory.

## Usage
- `govulncheck ./...` : scan the current module and its dependencies for vulnerabilities
- `govulncheck path/to/package` : scan a specific package
- `govulncheck -mode binary path/to/binary` : scan a Go binary instead of source code
- `govulncheck -mode extract path/to/binary` : extract build information and build dependencies from a Go binary
- `govulncheck -test ./...` : scan test files as well
- `govulncheck -format text|json|sarif|openvex ./...` : print the results in a specific format
- `govulncheck -show traces ./...` : display the call stacks leading to each vulnerable symbol

## Categories
packages, development, files

## Links
- More information: https://pkg.go.dev/golang.org/x/vuln/cmd/govulncheck
- tldr page: https://tldr.inbrowser.app/pages/common/govulncheck
