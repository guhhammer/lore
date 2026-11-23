# aa-cleanprof

Clean AppArmor security profiles by removing unused rules.

## Usage
- `sudo aa-cleanprof profile_name` : clean a profile to remove unused rules
- `sudo aa-cleanprof profile1 profile2 ...` : clean multiple profiles at once
- `sudo aa-cleanprof -d /path/to/profiles profile_name` : specify the directory containing profiles
- `sudo aa-cleanprof -s profile_name` : run silently without prompts
- `sudo aa-cleanprof --no-reload profile_name` : prevent profile reload after cleaning
- `aa-cleanprof -h` : display help

## Categories
users

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-cleanprof.8
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-cleanprof
