# auditctl

Utility to control the behavior, get status, and manage rules of the Linux Auditing System.

## Usage
- `sudo auditctl -s` : display the status of the audit system
- `sudo auditctl -l` : list all currently loaded audit rules
- `sudo auditctl -D` : delete all audit rules
- `sudo auditctl -e 1|0` : enable/disable the audit system
- `sudo auditctl -a always,exit -F arch=b64 -F path=/path/to/file -F perm=wa` : watch a file for changes
- `sudo auditctl -a always,exit -F arch=b64 -F dir=/path/to/directory/ -F perm=wa` : recursively watch a directory for changes
- `auditctl -h` : display help

## Categories
files, users

## Links
- More information: https://manned.org/auditctl
- tldr page: https://tldr.inbrowser.app/pages/linux/auditctl
