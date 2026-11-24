# asr

Restore (copy) a disk image onto a volume. The command name stands for Apple Software Restore.

## Usage
- `sudo asr restore --source path/to/image_file.dmg --target path/to/volume_file` : restore a disk image to a target volume
- `sudo asr restore --source path/to/image_file.dmg --target path/to/volume_file --erase` : erase the target volume before restoring
- `sudo asr restore --source path/to/image_file.dmg --target path/to/volume_file --noverify` : skip verification after restoring
- `sudo asr restore --source path/to/source_volume --target path/to/target_volume` : clone volumes without using an intermediate disk image

## Categories
files, disks, containers, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/asr.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/asr
- Wikipedia: https://en.wikipedia.org/wiki/Apple_Software_Restore
