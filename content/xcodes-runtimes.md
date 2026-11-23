# xcodes runtimes

Manage Xcode Simulator runtimes.

## Usage
- `xcodes runtimes --include-betas` : display all available Simulator runtimes
- `xcodes runtimes download runtime_name` : download a Simulator runtime
- `xcodes runtimes install runtime_name` : download and install a Simulator runtime
- `xcodes runtimes download|install "iOS|watchOS|tvOS|visionOS runtime_version"` : download/install a Simulator runtime for specific iOS/watchOS/tvOS/visionOS version (must be written as case-sensitive)
- `xcodes runtimes download|install runtime_name --directory path/to/directory` : set a specific location where the runtime archive will be first downloaded (defaults to `~/Downloads`)
- `xcodes runtimes install runtime_name --keep-archive` : do not delete the downloaded archive when the Simulator is successfully installed

## Categories
network, packages, archives, macos

## Links
- More information: https://github.com/xcodesorg/xcodes#commands
- tldr page: https://tldr.inbrowser.app/pages/osx/xcodes-runtimes
