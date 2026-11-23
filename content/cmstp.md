# cmstp

Manage connection service profiles.

## Usage
- `cmstp "path\to\profile_file"` : install a specific profile
- `cmstp /ns "path\to\profile_file"` : install without creating a desktop shortcut
- `cmstp /nf "path\to\profile_file"` : install without checking for dependencies
- `cmstp /su "path\to\profile_file"` : only install for the current user
- `cmstp /au "path\to\profile_file"` : install for all users (requires administrator privileges)
- `cmstp /s "path\to\profile_file"` : install silently without any prompts
- `cmstp /u "path\to\profile_file"` : uninstall a specific profile
- `cmstp /u /s "path\to\profile_file"` : uninstall silently without a confirmation prompt

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, packages, system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/cmstp
