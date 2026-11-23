# bclm

Set a custom charge limit on MacBooks.

## Usage
- `sudo bclm write 77` : set the charge limit to about 80% (for Intel machines, the battery charge level may be slightly lower than the set value)
- `bclm read` : read the current charge limit
- `sudo bclm persist` : keep the charge limit after rebooting/smc reset
- `sudo bclm unpersist` : remove the persistent charge limit

## Categories
users, macos

## Links
- More information: https://github.com/zackelia/bclm#usage
- tldr page: https://tldr.inbrowser.app/pages/osx/bclm
