# set-service

Start, stop, and suspend a service, and changes its properties. Note: This command can only be used through PowerShell.

## Usage
- `Set-Service -Name hostname -DisplayName "name"` : change a display name
- `Set-Service -Name service_name -StartupType Automatic` : change the startup type of services
- `Set-Service -Name service_name -Description "description"` : change the description of a service

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/set-service
