# checkov

Static code analysis tool for Infrastructure as Code (IaC). It is also a software composition analysis (SCA) tool for images and open source packages.

## Usage
- `checkov --directory path/to/directory` : scan a directory containing IaC (Terraform, Cloudformation, ARM, Ansible, Bicep, Dockerfile, etc.)
- `checkov --compact --file path/to/file` : scan an IaC file, omitting code blocks in the output
- `checkov --list` : list all checks for all IaC types

## Categories
files, containers, media

## Links
- More information: https://www.checkov.io/1.Welcome/Quick%20Start.html
- tldr page: https://tldr.inbrowser.app/pages/common/checkov
