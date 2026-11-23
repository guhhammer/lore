# get-wuhistory

Obtiene el historial de actualizaciones instaladas desde Windows Update. Parte del módulo externo `PSWindowsUpdate`. Este comando solo se puede ejecutar en PowerShell.

## Usage
- `Get-WUHistory` : obtiene la lista del historial de actualizaciones
- `Get-WUHistory -Last 10` : lista las últimas 10 actualizaciones instaladas
- `Get-WUHistory -MaxDate fecha` : lista todas las actualizaciones instaladas desde una fecha específica hasta hoy
- `Get-WUHistory -MaxDate (Get-Date).AddDays(-1)` : lista todas las actualizaciones instaladas en las últimas 24 horas
- `Get-WUHistory -SendReport -PSWUSettings @{SmtpServer="servidor_smtp"; Port=puerto_smtp From="correo_remitente" To="correo_destinatario"}` : envía los resultados por correo electrónico (SMTP)

## Links
- More information: https://github.com/mgajda83/PSWindowsUpdate
