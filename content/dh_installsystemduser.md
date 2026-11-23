# dh_installsystemduser

Install systemd unit files.

## Usage
- `dh_installsystemduser [debhelper options] [--no-enable] [--name=name] [unit file ...]` : the general form, as the manual gives it
- `dh_installsystemduser --no-enable` : disable the service(s) on purge, but do not enable them on install

## Categories
packages, system
