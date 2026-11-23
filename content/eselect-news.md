# eselect news

An `eselect` module for reading Gentoo news items. Note: Portage will print a notice when a repository is synchronized and there are unread news items.

## Usage
- `eselect news list all|new` : list available news items with their numbers (all by default)
- `eselect news read number1 number2 ...` : print the specified news items
- `eselect news read` : print all unread news items
- `eselect news unread number1 number2 ...` : mark the specified news items as unread
- `eselect news purge` : delete all read news items
- `eselect news count all|new` : print the number of available news items (new by default)

## Categories
documents, development, packages

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#News
- tldr page: https://tldr.inbrowser.app/pages/linux/eselect-news
