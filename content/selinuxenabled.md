# selinuxenabled

Check whether SELinux is enabled. Returns exit code 0 if SELinux is enabled, and 1 if it is not.

## Usage
- `selinuxenabled` : check if SELinux is enabled (no output; check exit code with `echo $?`)
- `selinuxenabled && echo "SELinux is enabled" || echo "SELinux is disabled"` : check if SELinux is enabled and print the result
- `if selinuxenabled; then echo "SELinux is running"; fi` : use in a shell script to conditionally execute commands

## See also
- getenforce
- setenforce
- sestatus

## Links
- More information: https://manned.org/selinuxenabled
- tldr page: https://tldr.inbrowser.app/pages/linux/selinuxenabled
