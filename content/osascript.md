# osascript

Run AppleScript or JavaScript for Automation (JXA).

## Usage
- `osascript -e 'say "Hello world"'` : run an AppleScript command
- `osascript -e 'say "Hello"' -e 'say "world"'` : run multiple AppleScript commands
- `osascript path/to/apple.scpt` : run a compiled (`*.scpt`), bundled (`*.scptd`), or plaintext (`*.applescript`) AppleScript file
- `osascript -e 'id of app "Application"'` : get the bundle identifier of an application (useful for `open -b`)
- `osascript -l JavaScript -e "console.log('Hello world');"` : run a JavaScript command
- `osascript -l JavaScript path/to/script.js` : run a JavaScript file

## Categories
files, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/osascript.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/osascript
