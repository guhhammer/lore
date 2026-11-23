# ntp-ctl

Management client for the `ntpd-rs` daemon.

## Usage
- `ntp-ctl status` : display information about the current state of the NTP daemon
- `ntp-ctl -c path/to/config validate` : check if the specified configuration file (default: `/etc/ntpd-rs/ntp.toml`) is valid
- `sudo ntp-ctl force-sync` : interactively run a single synchronization of the clock

## Categories
system

## Links
- More information: https://docs.ntpd-rs.pendulum-project.org/man/ntp-ctl.8/
- tldr page: https://tldr.inbrowser.app/pages/common/ntp-ctl
