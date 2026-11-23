# aura

The Aura Package Manager: a secure, multilingual package manager for Arch Linux and the AUR.

## Usage
- `aura -As keyword|regex` : search for packages from the AUR
- `aura -A package` : install a package from the AUR
- `aura -Akua` : update all AUR packages in a verbose mode and remove all make dependencies
- `aura -S package` : install a package from the official repositories
- `aura -Syu` : synchronize and update all packages from the official repositories
- `aura -Rsu package` : remove a package and its dependencies
- `aura -Oj` : remove orphan packages (installed as dependencies but not required by any package)

## Categories
packages, security

## Links
- More information: https://github.com/fosskers/aura
- tldr page: https://tldr.inbrowser.app/pages/linux/aura
