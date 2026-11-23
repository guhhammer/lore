# spctl

Manage the security assessment policy subsystem. Utility for managing Gatekeeper in macOS.

## Usage
- `spctl --master-disable` : turn off Gatekeeper
- `spctl --add --label rule_name path/to/file` : add a rule to allow an application to run (labeling of rule is optional)
- `spctl --master-enable` : turn on Gatekeeper
- `spctl --list` : list all rules on the system

## Categories
macos

## Links
- More information: https://keith.github.io/xcode-man-pages/spctl.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/spctl
