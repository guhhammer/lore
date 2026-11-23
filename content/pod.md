# pod

Dependency manager for Swift and Objective-C Cocoa projects.

## Usage
- `pod init` : create a Podfile for the current project with the default contents
- `pod install` : download and install all pods defined in the Podfile (that haven't been installed before)
- `pod list` : list all available pods
- `pod outdated` : show the outdated pods (of those currently installed)
- `pod update` : update all currently installed pods to their newest version
- `pod update pod_name` : update a specific (previously installed) pod to its newest version
- `pod deintegrate xcode_project` : remove CocoaPods from a Xcode project

## Categories
containers, network, macos

## Links
- More information: https://guides.cocoapods.org/terminal/commands.html
- tldr page: https://tldr.inbrowser.app/pages/osx/pod
