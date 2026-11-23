# kmutil

Utility for managing kernel extensions (kexts) and kext collections on disk.

## Usage
- `kmutil find` : find kexts available on the operating system
- `kmutil log` : display logging information about the Kernel Management sub-system
- `kmutil inspect options` : inspect and display a kext collection's contents according to the options provided
- `kmutil check` : check the consistency of kext collections against each other
- `sudo kmutil dumpstate` : dump kernelmanagerd state for debugging
- `kmutil load --bundle-path path/to/extension.kext` : load one or more extensions based on the bundle specified at this path in the results

## Categories
system, disks, files, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/kmutil.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/kmutil
