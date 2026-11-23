# vet

Scan package manifests, directories, container images, or GitHub repositories to identify vulnerabilities, malicious packages, and enforce security policies using CEL expressions.

## Usage
- `vet scan -D .` : scan the current directory
- `vet scan -M path/to/manifest_file` : scan the `package-lock.json` manifest file
- `vet scan -D path/to/directory --filter 'vulns.critical.exists(p, true)' --filter-fail` : scan with filter to fail on any critical vulnerability found in the codebase
- `vet inspect malware --purl package_url` : scan any OSS package for malware
- `vet server mcp` : start the MCP server for AI enabled security in code editors like Cursor

## Categories
files, packages, containers

## Links
- More information: https://safedep.github.io/vet/vet.html
- tldr page: https://tldr.inbrowser.app/pages/common/vet
