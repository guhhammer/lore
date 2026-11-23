# flatpak

Build, install, and run flatpak applications and runtimes.

## Usage
- `flatpak run com.example.app` : run an installed application
- `flatpak install remote_name com.example.app` : install an application from a remote source
- `flatpak list --app` : list installed applications, ignoring runtimes
- `flatpak update` : update all installed applications and runtimes
- `flatpak remote-add --if-not-exists remote_name remote_url` : add a remote source
- `flatpak remove com.example.app` : remove an installed application
- `flatpak remove --unused` : remove all unused applications
- `flatpak info com.example.app` : show information about an installed application

## Categories
packages

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak
