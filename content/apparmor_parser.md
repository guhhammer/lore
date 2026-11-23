# apparmor_parser

Load, compile, and manage AppArmor security profiles.

## Usage
- `sudo apparmor_parser -a profile_file` : load a profile into the kernel
- `sudo apparmor_parser -r profile_file` : replace an existing profile
- `sudo apparmor_parser -R profile_name` : remove a profile from the kernel
- `sudo apparmor_parser -C -r path/to/profile` : load a profile in complain mode (logs violations but doesn't block)
- `apparmor_parser -p -o path/to/output.cache -Q path/to/profile` : preprocess a profile (resolve includes) and write binary cache to file
- `apparmor_parser -p -S -Q path/to/profile` : preprocess and print binary profile to `stdout` without loading
- `sudo apparmor_parser -r -T path/to/profile` : replace a profile while skipping cache reads
- `sudo apparmor_parser -r -W -L /path/to/cache path/to/profile` : replace a profile, rebuild cache, and write it to a custom directory

## Categories
files, users, monitoring

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_apparmor_parser.8
- tldr page: https://tldr.inbrowser.app/pages/linux/apparmor_parser
