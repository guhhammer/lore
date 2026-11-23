# start-service

Start stopped services. Note: This command can only be used through PowerShell.

## Usage
- `Start-Service -Name service_name` : start a service by using its name
- `Start-Service -DisplayName *name* -WhatIf` : simulate starting a service, displaying what would happen
- `Set-Service service_name -StartupType manual; Start-Service service_name` : start a disabled service

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/start-service
