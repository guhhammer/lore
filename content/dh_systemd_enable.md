# dh_systemd_enable

Enable/disable systemd unit files.

## Usage
- `dh_systemd_enable [debhelper options] [--no-enable] [--name=name] [unit file ...]` : the general form, as the manual gives it
- `dh_systemd_enable --no-enable` : disable the service(s) on purge, but do not enable them on install

## Categories
system
