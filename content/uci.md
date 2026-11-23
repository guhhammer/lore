# uci

Manage OpenWrt configuration files.

## Usage
- `uci get network.lan.ipaddr` : fetch a value
- `uci show network` : list all options and their values
- `uci set config.section.option=value` : set a value
- `uci add config section` : add a new section
- `uci delete config.section.option` : delete a section or value
- `uci commit config` : commit changes
- `uci revert config` : discard uncommitted changes
- `uci` : display help

## Categories
network

## Links
- More information: https://openwrt.org/docs/techref/uci
- tldr page: https://tldr.inbrowser.app/pages/linux/uci
