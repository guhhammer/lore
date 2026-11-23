# machinectl

Control the systemd machine manager. Execute operations on virtual machines, containers, and images.

## Usage
- `sudo machinectl start machine_name` : start a machine as a service using `systemd-nspawn`
- `sudo machinectl stop machine_name` : stop a running machine
- `machinectl list` : display a list of running machines
- `sudo machinectl shell machine_name` : open an interactive shell inside the machine

## Categories
containers, system, media

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/machinectl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/machinectl
