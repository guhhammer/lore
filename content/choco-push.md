# choco push

Push a compiled NuGet package (`.nupkg`) to a package feed.

## Usage
- `choco push -s https://push.chocolatey.org/` : push a compiled `.nupkg` to the specified feed
- `choco push -s https://push.chocolatey.org/ --timeout 500` : push a compiled `.nupkg` to the specified feed with a timeout in seconds (default is 2700)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, network, windows

## Links
- More information: https://docs.chocolatey.org/en-us/create/commands/push/
- tldr page: https://tldr.inbrowser.app/pages/windows/choco-push
