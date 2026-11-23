# switcherooctl

Launch a command on a specific GPU.

## Usage
- `systemctl start switcheroo-control` : enable the switcheroo service
- `switcherooctl` : list known GPUs
- `switcherooctl launch command` : launch a command on the first discrete GPU
- `switcherooctl launch -g device_number command` : launch a command on a specific GPU
- `switcherooctl help` : display help

## Categories
hardware

## Links
- More information: https://manned.org/switcherooctl
- tldr page: https://tldr.inbrowser.app/pages/linux/switcherooctl
