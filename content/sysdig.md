# sysdig

System troubleshooting, analysis, and exploration. Capture, filter, and store systemcalls.

## Usage
- `sysdig` : capture all the events from the live system and print them to screen
- `sysdig -w path/to/file.scap` : capture all the events from the live system and save them to disk
- `sysdig -r path/to/file.scap` : read events from a file and print them to screen
- `sysdig proc.name=cat and evt.type=open` : filter and Print all the open system calls invoked by cat
- `sysdig -I dummy:'parameter'` : register any found plugin and use dummy as input source passing to it open params
- `sysdig -cl` : list the available chisels
- `sysdig -c spy_ip ip_address` : use the spy_ip chisel to look at the data exchanged with ip address

## Categories
files, documents, network

## Links
- More information: https://github.com/draios/sysdig/wiki
- tldr page: https://tldr.inbrowser.app/pages/linux/sysdig
