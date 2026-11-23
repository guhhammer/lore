# systemd-tmpfiles

Create, delete, and clean up volatile and temporary files and directories. This command is automatically invoked on boot by systemd services and running it manually is usually not needed.

## Usage
- `sudo systemd-tmpfiles --create` : create files and directories as specified in the configuration
- `sudo systemd-tmpfiles --create --prefix /var/log/journal` : create files and directories matching only a specific path prefix
- `sudo systemd-tmpfiles --clean` : clean up files and directories with age parameters configured
- `sudo systemd-tmpfiles --remove` : remove files and directories as specified in the configuration
- `sudo systemd-tmpfiles --create --user` : apply operations for user-specific configurations
- `sudo systemd-tmpfiles --create --boot` : execute lines marked for early boot

## Categories
system, files, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-tmpfiles
