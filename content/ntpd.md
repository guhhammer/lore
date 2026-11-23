# ntpd

The official NTP (Network Time Protocol) daemon to synchronize the system clock to remote time servers or local reference clocks.

## Usage
- `sudo ntpd` : start the daemon
- `sudo ntpd --quit` : synchronize system time with remote servers a single time (quit after synchronizing)
- `sudo ntpd --panicgate --quit` : synchronize a single time allowing "Big" adjustments

## Categories
system, network, users

## Links
- More information: https://manned.org/ntpd
- tldr page: https://tldr.inbrowser.app/pages/linux/ntpd
