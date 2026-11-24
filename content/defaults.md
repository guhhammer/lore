# defaults

Read and write macOS user configuration for applications.

## Usage
- `defaults read "application" "option"` : read system defaults for an application option
- `defaults read -app "application" "option"` : read default values for an application option
- `defaults find "keyword"` : search for a keyword in domain names, keys, and values
- `defaults write "application" "option" -type value` : write the default value of an application option
- `defaults write com.apple.Dock expose-animation-duration -float 0.1` : speed up Mission Control animations
- `defaults delete "application"` : delete all defaults of an application

## Categories
users, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/defaults.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/defaults
- Wikipedia: https://en.wikipedia.org/wiki/Defaults_(software)
