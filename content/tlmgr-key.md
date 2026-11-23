# tlmgr key

Manage GPG keys used to verify TeX Live databases.

## Usage
- `tlmgr key list` : list all keys for TeX Live
- `sudo tlmgr key add path/to/key.gpg` : add a key from a specific file
- `cat path/to/key.gpg | sudo tlmgr key add -` : add a key from `stdin`
- `sudo tlmgr key remove key_id` : remove a specific key by its ID

## Categories
security, files, users

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#key
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-key
