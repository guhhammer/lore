# dh_installsystemd

Install systemd unit files.

## Usage
- `dh_installsystemd -n` : do not modify postinst/postrm/prerm scripts
- `dh_installsystemd --no-enable` : disable the service(s) on purge, but do not enable them on install
- `dh_installsystemd --name=name` : this option controls several things
- `dh_installsystemd -r` : do not stop service on upgrade

## Categories
packages, system
