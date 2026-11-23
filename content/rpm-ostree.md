# rpm-ostree

A hybrid image/package system. Manage ostree deployments, package layers, filesystem overlays, and boot configuration.

## Usage
- `rpm-ostree status` : show rpm-ostree deployments in the order they will appear in the bootloader
- `rpm-ostree upgrade --preview` : show packages which are outdated and can be updated
- `rpm-ostree upgrade -r` : prepare a new ostree deployment with upgraded packages and reboot into it
- `rpm-ostree rollback -r` : reboot into the previous ostree deployment
- `rpm-ostree install package -r` : install a package into a new ostree deployment and reboot into it

## Categories
packages, system, containers

## Links
- More information: https://coreos.github.io/rpm-ostree/administrator-handbook/
- tldr page: https://tldr.inbrowser.app/pages/linux/rpm-ostree
