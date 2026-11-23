# flatpak history

Show historical changes to Flatpak installations on the system.

## Usage
- `flatpak history` : display all Flatpak history entries
- `flatpak history -u` : display history entries for the current user instead of system-wide
- `flatpak history --since time` : display history entries that occurred since a specific date or time
- `flatpak history --until time` : display history entries that occurred until a specific date or time
- `flatpak history --reverse` : display history entries with the newest changes first
- `flatpak history --columns application,change,time,...` : show specific columns of information

## See also
- journalctl

## Categories
shell, desktop

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak-history
