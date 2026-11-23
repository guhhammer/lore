# eselect locale

An `eselect` module for managing the `$LANG` environment variable, which sets the system language.

## Usage
- `eselect locale list` : list available locales
- `eselect locale set name|index` : set the `$LANG` environment variable in `/etc/profile.env` by name or index from the `list` command
- `eselect locale show` : display the value of `$LANG` in `/etc/profile.env`

## Categories
system, shell, development

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#Locale
- tldr page: https://tldr.inbrowser.app/pages/linux/eselect-locale
