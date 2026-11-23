# osv-scanner

Scan various mediums for dependencies and matches them against the OSV database.

## Usage
- `osv-scanner -D docker_image_name` : scan a Docker image
- `osv-scanner -L path/to/lockfile` : scan a package lockfile
- `osv-scanner -S path/to/sbom_file` : scan an SBOM file
- `osv-scanner -r directory1 directory2 ...` : scan multiple directories recursively
- `osv-scanner --skip-git -r|-D target` : skip scanning Git repositories
- `osv-scanner --json -D|-L|-S|-r target` : output result in JSON format

## Categories
files, packages, databases

## Links
- More information: https://google.github.io/osv-scanner/usage/
- tldr page: https://tldr.inbrowser.app/pages/common/osv-scanner
