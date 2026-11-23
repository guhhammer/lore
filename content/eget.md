# eget

Easily install prebuilt binaries from GitHub.

## Usage
- `eget zyedidia/micro` : download a prebuilt binary for the current system from a repository on GitHub
- `eget https://go.dev/dl/go1.17.5.linux-amd64.tar.gz` : download from a URL
- `eget zyedidia/micro --to path/to/directory` : specify the location to place the downloaded files
- `eget zyedidia/micro -t v2.0.10` : specify a Git tag instead of using the latest version
- `eget zyedidia/micro --pre-release` : install the latest pre-release instead of the latest stable version
- `eget zyedidia/micro -d` : only download the asset, skipping extraction
- `eget zyedidia/micro --upgrade-only` : only download if there is a newer release then the currently downloaded version

## Categories
network, packages

## Links
- More information: https://github.com/zyedidia/eget#usage
- tldr page: https://tldr.inbrowser.app/pages/common/eget
