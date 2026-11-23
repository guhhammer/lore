# aa-update-browser

Update AppArmor browser profiles to use supported abstractions. Part of the AppArmor suite.

## Usage
- `sudo aa-update-browser -l` : list available browser abstraction profiles
- `sudo aa-update-browser -d path/to/profile` : show what changes would be made to a profile without applying them (dry-run)
- `sudo aa-update-browser -u abstraction1,abstraction2,... path/to/profile` : update a profile with specific abstractions
- `sudo aa-update-browser -u '' path/to/profile` : remove all abstractions from a profile
- `aa-update-browser -h` : display help

## Categories
users, files

## Links
- More information: https://manned.org/aa-update-browser
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-update-browser
