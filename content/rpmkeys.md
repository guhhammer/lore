# rpmkeys

Tool to import and remove RPM keys for RPM repositories. When adding an RPM repository, you must also import the corresponding RPM key.

## Usage
- `sudo rpmkeys --list` : list all imported RPM keys. Also outputs its Key ID needed for deleting an imported RPM key
- `sudo rpmkeys --delete 5a278d9c-5bbc73cb` : remove/Delete a previously imported RPM key, given by its 16-Number/Letters Key ID
- `sudo rpmkeys --import path/to/rpm_key` : import an RPM key of repository

## Categories
packages, users

## Links
- More information: https://rpm-software-management.github.io/rpm/man/rpmkeys.8
- tldr page: https://tldr.inbrowser.app/pages/linux/rpmkeys
