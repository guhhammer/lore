# dnf install

Install packages on Red Hat-based distributions.

## Usage
- `sudo dnf in package1 package2 ...` : install packages by name
- `sudo dnf in path/to/file` : install a package from a local file
- `sudo dnf in https://example.com/package.rpm` : install a package from the internet
- `sudo dnf in https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm` : add the Fedora non-free packages repository
- `sudo dnf in epel-release` : add the Extra Packages for Enterprise Linux (EPEL) repositories
- `sudo dnf in https://rpms.remirepo.net/enterprise/remi-release-8.rpm` : add Remi's RPM repository

## Categories
packages, users, network

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#install-examples
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-install
