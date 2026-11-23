# pvchange

Change attributes of physical volume(s).

## Usage
- `sudo pvchange -x y /dev/sdXN` : allow allocation on a physical volume
- `sudo pvchange -x n /dev/sdXN` : disallow allocation on a physical volume
- `sudo pvchange --metadataignore y /dev/sdXN` : ignore metadata areas on a physical volume
- `sudo pvchange --metadataignore n /dev/sdXN` : stop ignoring metadata areas on a physical volume
- `sudo pvchange --addtag tag /dev/sdXN` : add a tag to a physical volume
- `sudo pvchange --uuid /dev/sdXN` : generate a new UUID for a physical volume (use with care)
- `sudo pvchange -a -x y` : change all visible physical volumes (combine with other options such as allocatable)

## Categories
disks, users

## Links
- More information: https://manned.org/pvchange
- tldr page: https://tldr.inbrowser.app/pages/linux/pvchange
