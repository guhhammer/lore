# update-alternatives

Conveniently maintain symbolic links to determine default commands.

## Usage
- `sudo update-alternatives --install path/to/symlink command_name path/to/command_binary priority` : add a symbolic link
- `sudo update-alternatives --config java` : configure a symbolic link for `java`
- `sudo update-alternatives --remove java /opt/java/jdk1.8.0_102/bin/java` : remove a symbolic link
- `update-alternatives --display java` : display information about a specified command
- `update-alternatives --get-selections` : display all commands and their current selection

## Categories
files, desktop, users

## Links
- More information: https://manned.org/update-alternatives
- tldr page: https://tldr.inbrowser.app/pages/linux/update-alternatives
