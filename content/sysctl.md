# sysctl

List and change kernel runtime variables.

## Usage
- `sysctl -a` : show all available variables and their values
- `sysctl -w section.tunable=value` : set a changeable kernel state variable
- `sysctl fs.file-nr` : get currently open file handlers
- `sysctl fs.file-max` : get limit for simultaneous open files
- `sysctl variable_name` : get the value of any runtime variable
- `sysctl -n variable_name` : print only the value without the variable name
- `sysctl -p` : apply changes from `/etc/sysctl.conf`

## Categories
system, files

## Links
- More information: https://manned.org/sysctl.8
- tldr page: https://tldr.inbrowser.app/pages/linux/sysctl
- Wikipedia: https://en.wikipedia.org/wiki/Sysctl
