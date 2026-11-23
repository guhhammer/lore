# newrole

Run a new shell with a different SELinux role. Allows users to switch to a different SELinux security context.

## Usage
- `newrole -r role_name` : start a new shell with a specific SELinux role
- `newrole -t type_name` : start a new shell with a specific SELinux type
- `newrole -l s0-s0:c0.c1023` : start a new shell with a specific SELinux level (format: `s0-s0:c0.c1023` where levels range from `s0` to `s15`, `-` indicates level range, categories start with `c`, `:` separates level from categories, `.` indicates category range)
- `id -Z` : display the current SELinux context
- `newrole -r role_name -t type_name` : start a new shell with both role and type

## See also
- runcon
- semanage user

## Categories
shell, users

## Links
- More information: https://manned.org/newrole
- tldr page: https://tldr.inbrowser.app/pages/linux/newrole
