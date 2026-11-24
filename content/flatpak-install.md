# flatpak install

Install flatpak applications and runtimes.

## Usage
- `flatpak install remote_name com.example.app` : install an application from a remote source
- `flatpak install -u remote_name com.example.app` : install an application for the current user only
- `flatpak install remote_name com.example.app//branch` : install a specific branch of an application
- `flatpak install --from path/to/file.flatpakref|https://example.com/file.flatpakref` : install an application from a `.flatpakref` file or URL
- `flatpak install --bundle path/to/file.flatpak` : install an application from a `.flatpak` bundle file
- `flatpak install --reinstall remote_name com.example.app` : reinstall an application that is already installed
- `flatpak install --or-update remote_name com.example.app` : update an application instead of warning if it is already installed
- `flatpak install -y remote_name com.example.app` : install an application without prompting for confirmation

## See also
- flatpak
- flatpak update
- flatpak run

## Categories
packages, files, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak-install
- Wikipedia: https://en.wikipedia.org/wiki/Flatpak
