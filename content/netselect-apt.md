# netselect-apt

Create a `sources.list` file for a Debian mirror with the lowest latency.

## Usage
- `sudo netselect-apt` : create `sources.list` using the lowest latency server
- `sudo netselect-apt testing` : specify Debian branch, stable is used by default
- `sudo netselect-apt -n` : include non-free section
- `sudo netselect-apt -c India` : specify a country for the mirror list lookup

## Categories
users, files

## Links
- More information: https://manned.org/netselect-apt
- tldr page: https://tldr.inbrowser.app/pages/linux/netselect-apt
