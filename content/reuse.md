# reuse

Tool for compliance with the REUSE recommendations.

## Usage
- `reuse lint` : lint for REUSE compliance for the current project (version control aware)
- `reuse --root path/to/directory lint` : lint for REUSE compliance from the specified directory
- `reuse annotate -c "your_name <your_email>" --fallback-dot-license path/to/file` : add copyright statement to file
- `reuse annotate -l spdx_identifier --fallback-dot-license path/to/file` : add license information to file
- `reuse download spdx_identifier` : download a license by its SPDX identifier and place it in the LICENSES directory
- `reuse download --all` : download all missing licenses detected in the project

## Categories
files, network

## Links
- More information: https://reuse.readthedocs.io/en/stable/man/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/reuse
