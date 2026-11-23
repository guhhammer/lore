# aurvote

Vote for packages in the Arch User Repository. To be able to vote, the file `~/.config/aurvote` must exist and contain your AUR credentials.

## Usage
- `aurvote --configure` : interactively create the file `~/.config/aurvote` containing your AUR username and password
- `aurvote package1 package2 ...` : vote for one or more AUR packages
- `aurvote -u package1 package2 ...` : unvote one or more AUR packages
- `aurvote -c package1 package2 ...` : check if one or more AUR packages have already been voted
- `aurvote -h` : display help

## Categories
packages, files, users

## Links
- More information: https://github.com/archlinuxfr/aurvote#name
- tldr page: https://tldr.inbrowser.app/pages/linux/aurvote
