# dockutil

Manage macOS dock items.

## Usage
- `dockutil --add path/to/application` : add an application to the end of the current user's dock
- `dockutil --add /path/to/application --replacing 'dock_item_label'` : replace one application with another in the current user's dock
- `dockutil --add /path/to/directory --view grid|fan|list|auto --display folder|stack` : add a directory with view options and display it as a folder icon or stack
- `dockutil --add vnc://example_server.local --label 'Example VNC' --after dock_item_label` : add a URL dock item after another item
- `dockutil --remove 'dock_item_label'` : remove an application from the dock given its dock label name
- `dockutil --add '' --type spacer|small-spacer|flex-spacer --section apps --after dock_item_label` : add a spacer in a section after an application
- `dockutil --remove spacer-tiles` : remove all spacer tiles

## Categories
files, network, macos

## Links
- More information: https://github.com/kcrawford/dockutil
- tldr page: https://tldr.inbrowser.app/pages/osx/dockutil
