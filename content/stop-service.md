# stop-service

Stop running services. Note: This command can only be used through PowerShell.

## Usage
- `Stop-Service -Name service_name` : stop a service on the local computer
- `Stop-Service -DisplayName "name"` : stop a service by using the display name
- `Stop-Service -Name service_name -Force -Confirm` : stop a service that has dependent services

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/stop-service
