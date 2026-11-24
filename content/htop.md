# htop

Display dynamic real-time information about running processes. An enhanced version of `top`.

## Usage
- `htop` : start `htop`
- `htop -u username` : start `htop` displaying processes owned by a specific user
- `htop -t` : display processes hierarchically in a tree view to show the parent-child relationships
- `htop -s sort_item` : sort processes by a specified `sort_item` (use `htop --sort help` for available options)
- `htop -d 50` : start `htop` with the specified delay between updates, in tenths of a second (i.e. 50 = 5 seconds)
- `htop --readonly` : disable all system and process changing features
- `<F1>|<?>` : see interactive commands while running `htop`
- `<Tab>` : switch to a different tab

## See also
- top
- atop
- glances
- btop
- btm

## Categories
processes, monitoring

## Links
- More information: https://manned.org/htop
- tldr page: https://tldr.inbrowser.app/pages/common/htop
- Wikipedia: https://en.wikipedia.org/wiki/Htop
