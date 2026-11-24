# fsck

Check the integrity of a filesystem or repair it. The filesystem should be unmounted at the time the command is run.

## Usage
- `sudo fsck /dev/sdXN` : check filesystem `/dev/sdXN`, reporting any damaged blocks
- `sudo fsck -r /dev/sdXN` : check filesystem `/dev/sdXN`, reporting any damaged blocks and interactively letting the user choose to repair each one
- `sudo fsck -a /dev/sdXN` : check filesystem `/dev/sdXN`, reporting any damaged blocks and automatically repairing them

## Categories
disks, users

## Links
- More information: https://manned.org/fsck
- tldr page: https://tldr.inbrowser.app/pages/linux/fsck
- Wikipedia: https://en.wikipedia.org/wiki/Fsck
