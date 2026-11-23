# systemd-nspawn

Spawn a command or OS in a lightweight container.

## Usage
- `systemd-nspawn -D path/to/container_root` : run a command in a container
- `systemd-nspawn -b -D path/to/container_root` : run a full Linux-based OS in a container
- `systemd-nspawn -D path/to/container_root -a` : run the specified command as PID 2 in the container (as opposed to PID 1) using a stub init process
- `systemd-nspawn -M container_name --hostname container_host -D path/to/container_root` : specify the machine name and hostname

## Categories
system, containers, files

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-nspawn.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-nspawn
