# aa-mergeprof

Merge AppArmor security profile files into the profile directory.

## Usage
- `sudo aa-mergeprof file1 file2 ...` : merge one or more profile files into the default profile directory
- `sudo aa-mergeprof -d /path/to/profiles file1 file2 ...` : merge profile files into a specific directory
- `aa-mergeprof -h` : display help

## Categories
files, version-control

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-mergeprof.8
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-mergeprof
