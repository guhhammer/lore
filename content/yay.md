# yay

Yet Another Yogurt: build and install packages from the Arch User Repository.

## Usage
- `yay package_name|search_term` : interactively search and install packages from the repos and AUR
- `yay` : synchronize and update all packages from the repos and AUR
- `yay -S package --noconfirm` : install a new package from the repos and AUR and do not ask to confirm transactions
- `yay -Rns package` : remove an installed package and both its dependencies and configuration files
- `yay -Ss keyword` : search the package database for a keyword from the repos and AUR
- `yay -Yc` : remove orphaned packages (installed as dependencies but not required by any package)
- `yay -Scc` : clean `pacman` and `yay` caches (old package versions kept for rollback and downgrade purposes)
- `yay -Ps` : show statistics for installed packages and system health

## See also
- pacman

## Categories
packages, development, users

## Links
- More information: https://github.com/Jguer/yay#first-use
- tldr page: https://tldr.inbrowser.app/pages/linux/yay
