# audit2allow

Generate SELinux policy allow rules from audit logs. Part of the `policycoreutils-python-utils` package.

## Usage
- `sudo audit2allow -a` : generate allow rules from recent audit denials and display them
- `sudo audit2allow -i path/to/audit.log` : generate allow rules from a specific audit log file
- `sudo audit2allow -a -M module_name` : generate a policy module from recent audit denials
- `sudo audit2allow -a --why` : explain why SELinux denials occurred (same as `audit2why`)
- `sudo audit2allow -a -e` : display detailed information around generated messages
- `sudo audit2allow -a -R` : use installed macros to generate a reference policy
- `sudo ausearch -m avc -c service_name | audit2allow -M policy_name` : generate allow rules for a specific service
- `sudo audit2allow -a -v` : enable verbose output mode

## See also
- audit2why
- ausearch
- semodule

## Categories
users, packages

## Links
- More information: https://manned.org/audit2allow
- tldr page: https://tldr.inbrowser.app/pages/linux/audit2allow
