# xctool

Build Xcode projects.

## Usage
- `xctool -project YourProject.xcodeproj -scheme YourScheme build` : build a single project without any workspace
- `xctool -workspace YourWorkspace.xcworkspace -scheme YourScheme build` : build a project that is part of a workspace
- `xctool -workspace YourWorkspace.xcworkspace -scheme YourScheme clean build test` : clean, build, and execute all the tests

## Categories
development, macos

## Links
- More information: https://github.com/facebookarchive/xctool#usage
- tldr page: https://tldr.inbrowser.app/pages/osx/xctool
