# flatpak run

Run flatpak applications and runtimes.

## Usage
- `flatpak run com.example.app` : run an installed application
- `flatpak run --branch=stable|beta|master|... com.example.app` : run an installed application from a specific branch e.g. stable, beta, master
- `flatpak run --command=sh com.example.app` : run an interactive shell inside a flatpak
- `flatpak run --runtime-version=24.08|master|stable|... com.example.app` : run an installed application with a specific runtime version
- `flatpak run --runtime=org.freedesktop.Sdk com.example.app` : run an installed application with a different runtime (but same version number)

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak-run
