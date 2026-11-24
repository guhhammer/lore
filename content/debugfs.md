# debugfs

An interactive ext2/ext3/ext4 filesystem debugger.

## Usage
- `debugfs /dev/sdXN` : open the filesystem in read only mode
- `debugfs -w /dev/sdXN` : open the filesystem in read write mode
- `debugfs -f path/to/cmd_file /dev/sdXN` : read commands from a specified file, execute them, and then exit
- `stats` : [Interactive] View the filesystem stats in debugfs console
- `close -a` : [Interactive] Close the filesystem
- `lr` : [Interactive] List all available commands

## Categories
disks, development

## Links
- More information: https://manned.org/debugfs
- tldr page: https://tldr.inbrowser.app/pages/linux/debugfs
- Wikipedia: https://en.wikipedia.org/wiki/Debugfs
