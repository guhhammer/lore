# systemd-delta

Find overridden systemd-related configuration files.

## Usage
- `systemd-delta` : show all overridden configuration files
- `systemd-delta -t masked|equivalent|redirected|overridden|extended|unchanged` : show only files of specific types (comma-separated list)
- `systemd-delta /etc|/run|/usr/lib|...` : show only files whose path starts with the specified prefix (Note: A prefix is a directory containing subdirectories with systemd configuration files)
- `systemd-delta prefix/tmpfiles.d|sysctl.d|systemd/system|...` : further restrict the search path by adding a suffix (the prefix is optional)

## Categories
system, files

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-delta.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-delta
