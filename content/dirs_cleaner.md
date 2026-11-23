# dirs_cleaner

Recursively empty directories while the directories themselves are not deleted. Note: On macOS 13 (Ventura) and later, `dirs_cleaner` is subject to AMFI launch constraints introduced in Ventura. The binary remains present at `/usr/libexec/dirs_cleaner` but cannot be invoked directly from a user session, including as root.

## Usage
- `sudo dirs_cleaner path/to/directory1 path/to/directory2 ...` : recursively delete contents of folder structure without removing directories

## Categories
files, users, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/dirs_cleaner.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/dirs_cleaner
