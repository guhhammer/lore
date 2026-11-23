# xcode-select

Switch between different versions of Xcode and the included developer tools. Also used to update the path to Xcode if it is moved after installation.

## Usage
- `xcode-select --install` : install Xcode's command-line tools
- `xcode-select --switch path/to/Xcode.app/Contents/Developer` : select a given path as the active developer directory
- `xcode-select --switch path/to/Xcode_file.app` : select a given Xcode instance and use its developer directory as the active one
- `xcode-select --print-path` : print the currently selected developer directory
- `sudo xcode-select --reset` : discard any user-specified developer directory so that it will be found via the default search mechanism

## Categories
files, macos

## Links
- More information: https://developer.apple.com/library/archive/technotes/tn2339/_index.html
- tldr page: https://tldr.inbrowser.app/pages/osx/xcode-select
