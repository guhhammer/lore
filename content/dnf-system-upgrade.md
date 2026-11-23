# dnf system-upgrade

Upgrade Fedora to a new major release.

## Usage
- `sudo dnf upgrade --refresh` : prepare the system by upgrading all installed packages
- `sudo dnf system-upgrade download --releasever release_version` : download packages required for the new release version
- `sudo dnf system-upgrade reboot` : reboot and start the upgrade process
- `sudo dnf system-upgrade status` : display the status of the current upgrade transaction
- `sudo dnf system-upgrade log` : display logs from previous upgrade attempts
- `sudo dnf system-upgrade clean` : remove cached upgrade data and packages

## Categories
packages, users, network

## Links
- More information: https://dnf5.readthedocs.io/en/latest/commands/system-upgrade.8.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-system-upgrade
