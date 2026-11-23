# enable-pnpdevice

El cmdlet Enable-PnpDevice habilita un dispositivo Plug and Play (PnP). Debe usar una cuenta de Administrador para habilitar un dispositivo. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Enable-PnpDevice -InstanceId 'ID OBTENIDO USANDO EL COMANDO Get-PnpDevice'` : habilita un dispositivo
- `Get-PnpDevice | Where-Object {$_.Problem -eq 22} | Enable-PnpDevice` : habilita todos los dispositivos PnP deshabilitados
- `Enable-PnpDevice -InstanceId 'ID OBTENIDO USANDO EL COMANDO Get-PnpDevice' -Confirm:$False` : habilita un dispositivo sin confirmación
- `Enable-PnpDevice -InstanceId 'USB\VID_5986&;PID_0266&;MI_00\7&;1E5D3568&;0&;0000' -WhatIf:$True` : simulación de lo que sucedería si se ejecuta el cmdlet

## Links
- More information: https://learn.microsoft.com/powershell/module/pnpdevice/enable-pnpdevice
