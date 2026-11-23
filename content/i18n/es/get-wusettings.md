# get-wusettings

Obtiene la configuración actual del Agente de Windows Update. Parte del módulo externo `PSWindowsUpdate`. Este comando solo se puede ejecutar en PowerShell.

## Usage
- `Get-WUSettings` : obtiene la configuración actual del Agente de Windows Update
- `Get-WUSettings -SendReport -PSWUSettings @{SmtpServer="servidor_smtp"; Port=puerto_smtp From="correo_remitente" To="correo_destinatario"}` : envía los datos de configuración actuales por correo electrónico (SMTP)

## Links
- More information: https://github.com/mgajda83/PSWindowsUpdate
