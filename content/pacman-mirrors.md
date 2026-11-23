# pacman-mirrors

Generate a `pacman` mirrorlist for Manjaro Linux. Every run of `pacman-mirrors` requires you to synchronize your database and update your system using `sudo pacman -Syyu`.

## Usage
- `sudo pacman-mirrors --fasttrack` : generate a mirrorlist using the default settings
- `pacman-mirrors --status` : get the status of the current mirrors
- `pacman-mirrors --get-branch` : display the current branch
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : switch to a different branch
- `pacman-mirrors --country-list` : list available countries
- `sudo pacman-mirrors --country United_States,France,Germany,...` : generate a mirrorlist using only mirrors in specific countries

## See also
- pacman

## Categories
users, version-control, databases

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-mirrors
