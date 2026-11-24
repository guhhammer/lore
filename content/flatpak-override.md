# flatpak override

Override the runtime permissions of Flatpak applications. Note: Without `--user`, the default system-wide installation is changed. Without an application, the override applies to all applications.

## Usage
- `flatpak override --show com.example.app` : show the overrides of an application
- `flatpak override --filesystem home|host|xdg-download|path/to/directory com.example.app` : grant an application access to a filesystem location
- `flatpak override --filesystem path/to/directory:ro com.example.app` : grant an application read-only access to a directory
- `flatpak override --unshare network com.example.app` : stop sharing the network with an application
- `flatpak override --env VARIABLE=value com.example.app` : set an environment variable for an application
- `flatpak override -u --nosocket wayland com.example.app` : apply an override for the current user only
- `flatpak override --reset com.example.app` : remove all overrides of an application

## Categories
files, network, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak-override
- Wikipedia: https://en.wikipedia.org/wiki/Flatpak
