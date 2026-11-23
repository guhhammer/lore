# octo

Tools for Octopus Deploy.

## Usage
- `octo pack --id package` : create a package
- `octo push --package package` : push a package to a repository on the Octopus server
- `octo create-release --project project_name --packageversion version` : create a release
- `octo deploy-release --project project_name --packageversion version --deployto environment_name --tenant deployment_target` : deploy a release

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, windows

## Links
- More information: https://octopus.com/docs/cli/octopus-cli
- tldr page: https://tldr.inbrowser.app/pages/windows/octo
