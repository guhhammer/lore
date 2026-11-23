# mirrorselect

Select the fastest Gentoo mirrors and update `GENTOO_MIRRORS` in `/etc/portage/make.conf`.

## Usage
- `sudo mirrorselect -i` : interactively select source mirrors from a list
- `sudo mirrorselect -i -c "Germany"` : interactively select mirrors from a specific country
- `sudo mirrorselect -s 3` : automatically select a specific number of the fastest mirrors
- `sudo mirrorselect -D -s 3` : automatically select the fastest mirrors using a more accurate download test
- `sudo mirrorselect -S -s 3` : only consider HTTPS mirrors
- `mirrorselect -s 3 -o` : print the selected mirrors instead of writing them to `make.conf`
- `mirrorselect -i -r -o | sudo tee -a /etc/portage/repos.conf/gentoo.conf` : interactively select an rsync mirror and append it to the repository configuration

## Categories
users, network

## Links
- More information: https://wiki.gentoo.org/wiki/Mirrorselect
- tldr page: https://tldr.inbrowser.app/pages/linux/mirrorselect
