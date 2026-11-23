# systemctl set-default

Symlink the `default.target` alias to the given target unit.

## Usage
- `systemctl set-default target_name.target` : set `systemd`'s default boot mode
- `systemctl set-default graphical.target` : set `systemd` to boot to GUI mode by default
- `systemctl set-default multi-user.target` : set `systemd` to boot to CLI mode by default

## Categories
system, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-set-default
