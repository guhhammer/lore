# carthage

A dependency management tool for Cocoa applications.

## Usage
- `carthage update` : download the latest version of all dependencies mentioned in Cartfile, and build them
- `carthage update --platform ios` : update dependencies, but only build for iOS
- `carthage update --no-build` : update dependencies, but don't build any of them
- `carthage bootstrap` : download and rebuild the current version of dependencies (without updating them)
- `carthage build dependency` : rebuild a specific dependency

## Categories
development, network, packages, macos

## Links
- More information: https://github.com/Carthage/Carthage
- tldr page: https://tldr.inbrowser.app/pages/osx/carthage
