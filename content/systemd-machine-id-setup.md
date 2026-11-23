# systemd-machine-id-setup

Initialize the machine ID stored in `/etc/machine-id` at install time with a provisioned or randomly generated ID. Note: Always use `sudo` to execute these commands as they require elevated privileges.

## Usage
- `systemd-machine-id-setup --print` : print the generated or committed machine ID
- `systemd-machine-id-setup --image-policy your_policy` : specify an image policy
- `sudo systemd-machine-id-setup --json pretty` : display the output as JSON
- `systemd-machine-id-setup --image /path/to/image` : operate on a disk image instead of a directory tree

## Categories
system, containers, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-machine-id-setup
