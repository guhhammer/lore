# flatpak uninstall

Uninstall flatpak applications and runtimes.

## Usage
- `flatpak uninstall com.example.app` : uninstall an application or runtime
- `flatpak uninstall -u com.example.app` : uninstall an application from the per-user installation
- `flatpak uninstall --delete-data com.example.app` : uninstall an application and delete its data in `~/.var/app`
- `flatpak uninstall --unused` : uninstall all unused applications and runtimes
- `flatpak uninstall --delete-data` : delete the data of all applications that are no longer installed
- `flatpak uninstall -y com.example.app` : uninstall an application without prompting for confirmation
- `flatpak uninstall --all` : uninstall all applications and runtimes

## See also
- flatpak install
- flatpak update

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak-uninstall
- Wikipedia: https://en.wikipedia.org/wiki/Flatpak
