# automount

Read the `/etc/auto_master` file and mount `autofs` on the appropriate mount points to trigger the on-demand mounting of directories. Essentially, it's a way to manually initiate the system's automounting process. Note: You'll most likely need to run with `sudo` if you don't have the necessary permissions.

## Usage
- `automount -cv` : run automount, flush the cache beforehand, and be verbose about it (most common use)
- `automount -t 300` : automatically unmount after 5 minutes (300 seconds) of inactivity
- `automount -u` : unmount anything previously mounted by automount and/or defined in `/etc/auto_master`

## Categories
disks, files, processes, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/automount.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/automount
