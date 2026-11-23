# flatpak ps

List running Flatpak instances.

## Usage
- `flatpak ps` : list running instances
- `flatpak ps --columns application,child-pid` : show the application ID and the process ID of the sandbox for each instance
- `flatpak ps --columns application,runtime,runtime-branch` : show the runtime and its branch alongside each application ID
- `flatpak ps --columns help` : list the available columns

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak-ps
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak-ps
