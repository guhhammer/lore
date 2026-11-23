# systemctl reload

Reload a service's configuration without restarting it. This reloads the service itself (like Apache or `nginx` configs), not the systemd unit file. To reload unit files, use `systemctl daemon-reload`.

## Usage
- `systemctl reload nginx` : reload a service
- `systemctl reload unit1 unit2 ...` : reload multiple services
- `systemctl reload pipewire --user` : reload a service for the current user

## Categories
system, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-reload
