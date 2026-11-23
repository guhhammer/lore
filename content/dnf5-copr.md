# dnf5 copr

Manage Copr repositories on Fedora-based systems.

## Usage
- `sudo dnf5 copr enable owner/project` : enable a Copr repository
- `sudo dnf5 copr enable owner/project fedora-rawhide-ppc64le` : enable a Copr repository for a specific chroot
- `sudo dnf5 copr enable hub/owner/project` : enable a Copr repository from a specific hub
- `sudo dnf5 copr disable owner/project` : disable a Copr repository
- `sudo dnf5 copr remove owner/project` : remove a Copr repository
- `dnf5 copr list` : list Copr repositories
- `dnf5 copr debug` : print debug information about the system

## Categories
development, packages, users

## Links
- More information: https://dnf5.readthedocs.io/en/latest/dnf5_plugins/copr.8.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf5-copr
