# rankmirrors

Rank a list of Pacman mirrors by connection and opening speed. Writes the new mirrorlist to `stdout`.

## Usage
- `rankmirrors /etc/pacman.d/mirrorlist` : rank a mirror list
- `rankmirrors -n number /etc/pacman.d/mirrorlist` : output only a given number of the top ranking servers
- `rankmirrors -v /etc/pacman.d/mirrorlist` : be verbose when generating the mirrorlist
- `rankmirrors -u url` : test only a specific URL
- `rankmirrors -t /etc/pacman.d/mirrorlist` : output only the response times instead of a full mirrorlist

## Categories
network

## Links
- More information: https://manned.org/rankmirrors
- tldr page: https://tldr.inbrowser.app/pages/linux/rankmirrors
