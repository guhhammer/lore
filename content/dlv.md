# dlv

Debugger for the Go programming language.

## Usage
- `dlv debug` : compile and begin debugging the main package in the current directory (by default, with no arguments)
- `dlv debug package arguments` : compile and begin debugging a specific package
- `dlv test` : compile a test binary and begin debugging the compiled program
- `dlv connect ip_address` : connect to a headless debug server
- `dlv attach process_id` : attach to a running process and begin debugging
- `dlv trace package --regexp 'regex'` : compile and begin tracing a program

## Categories
development, packages

## Links
- More information: https://github.com/go-delve/delve/blob/master/Documentation/usage/dlv.md
- tldr page: https://tldr.inbrowser.app/pages/common/dlv
