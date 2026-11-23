# quickemu

Build and manage highly optimized desktop virtual machines quickly. Note: Virtual machine must be in stopped state when working with snapshots.

## Usage
- `quickemu --vm path/to/file.conf` : create and run a virtual machine from a configuration file
- `quickemu --status-quo --vm path/to/file.conf` : do not commit any changes to disk/snapshot but write any changes to temporary files
- `quickemu --fullscreen --display sdl|gtk|spice|spice-app|none --vm path/to/file.conf` : start the virtual machine in full-screen mode (`<Ctrl Alt f>` to exit) and select the display backend (`sdl` by default)
- `quickemu --sound-card intel-hda|ac97|es1370|sb16|none --shortcut --vm path/to/file.conf` : select a virtual audio device to emulate and create a desktop shortcut
- `quickemu --snapshot create|apply|delete tag --vm path/to/file.conf` : create/restore/delete a snapshot
- `quickemu --snapshot info --vm path/to/file.conf` : list available snapshots
- `quickemu --delete-vm --vm path/to/file.conf` : delete the entire virtual machine and its configuration
- `quickemu --delete-disk --vm path/to/file.conf` : delete the virtual machine's disk image and EFI variables

## See also
- quickget

## Categories
files, desktop, development

## Links
- More information: https://github.com/quickemu-project/quickemu
- tldr page: https://tldr.inbrowser.app/pages/linux/quickemu
