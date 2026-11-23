# enable-pnpdevice

Enable a Plug and Play (PnP) device. You must use an Administrator account to enable a device. Note: This command can only be used through PowerShell.

## Usage
- `Enable-PnpDevice -InstanceId 'RETRIEVED USING Get-PnpDevice COMMAND'` : enable a device
- `Get-PnpDevice | Where-Object {$_.Problem -eq 22} | Enable-PnpDevice` : enable all disabled PnP devices
- `Enable-PnpDevice -InstanceId 'RETRIEVED USING Get-PnpDevice COMMAND' -Confirm:$False` : enable a device without confirmation
- `Enable-PnpDevice -InstanceId 'USB\VID_5986&;PID_0266&;MI_00\7&;1E5D3568&;0&;0000' -WhatIf:$True` : simulate what would happen if the cmdlet runs

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
hardware, users, windows

## Links
- More information: https://learn.microsoft.com/powershell/module/pnpdevice/enable-pnpdevice
- tldr page: https://tldr.inbrowser.app/pages/windows/enable-pnpdevice
