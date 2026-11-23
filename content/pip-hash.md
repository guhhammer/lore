# pip hash

Compute hashes of package archives for verification.

## Usage
- `pip hash path/to/package.whl` : generate hash for a package file
- `pip hash -a sha256|sha384|sha512|... path/to/package.whl` : generate hash using a specific algorithm
- `pip hash path/to/package1.whl path/to/package2.whl ...` : generate hashes for multiple files
- `pip hash path/to/package.tar.gz` : generate hash for downloaded archive

## Categories
security, packages, files

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_hash/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-hash
