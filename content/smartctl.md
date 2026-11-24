# smartctl

Monitor disk health including SMART data.

## Usage
- `sudo smartctl -H /dev/sdX` : display SMART health summary
- `sudo smartctl -i /dev/sdX` : display device information
- `sudo smartctl -t short|long /dev/sdX` : start a short/long self-test in the background
- `sudo smartctl -l selftest /dev/sdX` : display the self-test log
- `sudo smartctl -c /dev/sdX` : display current/last self-test status and other SMART capabilities
- `sudo smartctl -a /dev/sdX` : display exhaustive SMART data

## Categories
users, desktop, monitoring

## Links
- More information: https://manned.org/smartctl
- tldr page: https://tldr.inbrowser.app/pages/common/smartctl
- Wikipedia: https://en.wikipedia.org/wiki/Smartmontools
