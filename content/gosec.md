# gosec

Inspect Go source code for security problems. Note: `./...` is a Go package pattern understood by Go tooling. It matches the current package and all packages recursively under the current directory.

## Usage
- `gosec ./...` : scan a Go package recursively in the current directory
- `gosec path/to/package` : scan a specific Go package
- `gosec -fmt json|yaml|csv|html|sonarqube|golint|sarif|junit-xml -out path/to/report ./...` : scan a Go package recursively and save results to a file
- `gosec -include G101,G203,G401,... ./...` : only include a specific set of rules (by default, all rules will run)
- `gosec -exclude-dir path/to/excluded_directory ./...` : exclude directories from scanning
- `gosec -severity low|medium|high -confidence low|medium|high ./...` : scan with a specific severity or confidence level
- `gosec -exclude G101,G304,G401,... ./...` : exclude a specific set of rules from scanning
- `gosec -tests ./...` : scan test files as well

## Categories
packages, files, text

## Links
- More information: https://github.com/securego/gosec
- tldr page: https://tldr.inbrowser.app/pages/common/gosec
