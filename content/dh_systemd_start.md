# dh_systemd_start

Start/stop/restart systemd unit files.

## Usage
- `dh_systemd_start [debhelper options] [--restart-after-upgrade] [--no-stop-on-upgrade] [unit file ...]` : the general form, as the manual gives it
- `dh_systemd_start -r` : do not stop service on upgrade

## Categories
system, packages
