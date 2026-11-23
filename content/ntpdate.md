# ntpdate

Synchronize and set the date and time via NTP.

## Usage
- `sudo ntpdate host` : synchronize and set date and time
- `ntpdate -q host` : query the host without setting the time
- `sudo ntpdate -u host` : use an unprivileged port in case a firewall is blocking privileged ports
- `sudo ntpdate -b host` : force time to be stepped using `settimeofday` instead of `slewed`

## Categories
network, users

## Links
- More information: https://manned.org/ntpdate
- tldr page: https://tldr.inbrowser.app/pages/linux/ntpdate
