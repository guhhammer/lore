# synoupgrade

Upgrade Synology DiskStation Manager (DSM) - the Synology NAS operating system.

## Usage
- `sudo synoupgrade --check` : check if upgrades are available
- `sudo synoupgrade --check-smallupdate` : check for patches without upgrading the DSM version
- `sudo synoupgrade --download` : download the latest upgrade available (use `--download-smallupdate` for patches)
- `sudo synoupgrade --start` : start the upgrade process
- `sudo synoupgrade --auto` : upgrade to the latest version automatically
- `sudo synoupgrade --auto-smallupdate` : apply patches without upgrading the DSM version automatically
- `sudo synoupgrade --patch /path/to/file.pat` : upgrade the DSM using a patch file (should be an absolute path)
- `synoupgrade` : display help

## Categories
packages, users, network

## Links
- More information: https://www.synology.com/dsm
- tldr page: https://tldr.inbrowser.app/pages/linux/synoupgrade
