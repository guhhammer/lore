# pm list

List users, packages, permissions, instrumentation, permission groups, features, and libraries managed by the package manager.

## Usage
- `pm list packages` : list all installed packages
- `pm list packages -s|-3` : list either system or [3]rd-party packages
- `pm list users` : print all users on the system
- `pm list permission-groups` : print all known permission groups
- `pm list permissions` : print all known permissions
- `pm list instrumentation` : list all test packages
- `pm list features` : print all features of the system
- `pm list libraries` : print all the libraries supported by the current device

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
packages, users, documents, android

## Links
- More information: https://developer.android.com/tools/adb#pm
- tldr page: https://tldr.inbrowser.app/pages/android/pm-list
