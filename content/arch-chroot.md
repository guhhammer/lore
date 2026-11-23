# arch-chroot

Enhanced `chroot` command to help in the Arch Linux installation process.

## Usage
- `arch-chroot path/to/new_root` : start an interactive shell (Bash, by default) in a new root directory
- `arch-chroot -u user path/to/new_root` : specify the user (other than the current user) to run the shell as
- `arch-chroot path/to/new_root command command_arguments` : run a custom command (instead of the default Bash) in the new root directory
- `arch-chroot path/to/new_root zsh` : specify the shell, other than the default Bash (in this case, the `zsh` package should have been installed in the target system)

## Categories
files, processes, shell

## Links
- More information: https://manned.org/arch-chroot
- tldr page: https://tldr.inbrowser.app/pages/linux/arch-chroot
