# pkgctl repo

Manage Git packaging repositories and their configuration for Arch Linux.

## Usage
- `pkgctl repo clone pkgname` : clone a package repository (requires setting an SSH key in your Arch Linux GitLab account)
- `pkgctl repo clone --protocol https pkgname` : clone a package repository over HTTPS
- `pkgctl repo create pkgbase` : create a new GitLab package repository and clone it after creation (requires valid GitLab API authentication)
- `pkgctl repo switch version pkgbase` : switch a package repository to a specified version
- `pkgctl repo web pkgbase` : open a package repository's website

## Categories
packages, network, version-control

## Links
- More information: https://manned.org/pkgctl-repo
- tldr page: https://tldr.inbrowser.app/pages/linux/pkgctl-repo
