# sntpd

An SNTP server. Note: It should not be invoked manually.

## Usage
- `sntpd` : start the daemon
- `sntpd -L` : overwrite existing state with the local clock (stratum 1), for running a master/primary server, without synchronizing with another (higher stratum) server
- `sntpd -z path/to/state.bin` : use a custom file for the SNTP state

## Categories
macos

## Links
- More information: https://keith.github.io/xcode-man-pages/sntpd.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/sntpd
