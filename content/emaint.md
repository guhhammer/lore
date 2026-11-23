# emaint

Perform Portage maintenance tasks.

## Usage
- `sudo emaint sync -a` : synchronize repositories that are set to auto-sync (default for most repositories)
- `sudo emaint sync -r repository` : synchronize a specific repository
- `sudo emaint sync -A` : synchronize all repositories
- `sudo emaint cleanresume -f` : clear the Portage resume list
- `sudo emaint logs -C` : clean Portage logs

## Categories
users

## Links
- More information: https://wiki.gentoo.org/wiki/Portage#emaint
- tldr page: https://tldr.inbrowser.app/pages/linux/emaint
