# xcodebuild

Build Xcode projects.

## Usage
- `xcodebuild -workspace workspace_name.workspace -scheme scheme_name -configuration configuration_name clean build SYMROOT=path/to/SYMROOT` : build workspace
- `xcodebuild -target target_name -configuration configuration_name clean build SYMROOT=path/to/SYMROOT` : build project
- `xcodebuild -showsdks` : show SDKs

## Categories
development, macos

## Links
- More information: https://developer.apple.com/library/archive/technotes/tn2339/_index.html
- tldr page: https://tldr.inbrowser.app/pages/osx/xcodebuild
