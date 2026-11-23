# vso

Package manager, system updater, and a task automator for Vanilla OS.

## Usage
- `vso sys-upgrade check` : check for system updates to the host system
- `vso sys-upgrade upgrade --now` : upgrade the host system now
- `vso pico-init` : initialize the Pico subsystem (used for package management)
- `vso install package1 package2 ...` : install applications inside the subsystem
- `vso remove package1 package2 ...` : remove applications from the subsystem
- `vso shell` : enter the subsystem's shell
- `vso run package` : run an application from the subsystem
- `vso config show` : display VSO configuration

## Categories
packages, network

## Links
- More information: https://github.com/Vanilla-OS/vanilla-system-operator
- tldr page: https://tldr.inbrowser.app/pages/linux/vso
