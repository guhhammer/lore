# bun why

Explain why a package is installed by showing its dependency chain.

## Usage
- `bun why package_name` : explain why a specific package is installed
- `bun why "pattern"` : explain why all packages matching a pattern are installed
- `bun why "@organization/*"` : explain why packages from a specific organization are installed
- `bun why package_name --top` : show only top-level dependencies
- `bun why package_name --depth number` : limit the dependency tree depth

## Categories
packages

## Links
- More information: https://bun.com/docs/pm/cli/why
- tldr page: https://tldr.inbrowser.app/pages/common/bun-why
