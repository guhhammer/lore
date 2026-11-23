# copr-cli

Interface with Fedora-Projects copr instance for building RPMs and publishing them.

## Usage
- `copr-cli whoami` : show user logged in to copr
- `copr-cli build repository path/to/spec_file` : build a local spec file on copr
- `copr-cli list-builds repository` : check status of builds
- `copr-cli buildscm repository --clone-url https://git.example.org/repo --spec spec_file_name` : trigger a copr build of a spec-file from public (Git) repository

## Categories
network, packages, development

## Links
- More information: https://manned.org/copr-cli
- tldr page: https://tldr.inbrowser.app/pages/common/copr-cli
