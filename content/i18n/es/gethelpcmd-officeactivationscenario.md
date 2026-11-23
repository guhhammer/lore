# gethelpcmd officeactivationscenario

Repara automáticamente los problemas relacionados con la activación en Microsoft Office y las aplicaciones de Microsoft 365 para empresas. Forma parte de `GetHelpCmd.exe`, anteriormente `SaRAcmd.exe` (Asistente de soporte y recuperación de Microsoft). Nota: Esta herramienta ha quedado obsoleta y no funcionará en las nuevas aplicaciones de OneNote y Outlook. Nota: Este comando puede anular, desactivar o eliminar el volumen actual de versiones con licencia de los productos de Office, por lo que se recomienda proceder con precaución.

## Usage
- `GetHelpCmd.exe -S OfficeActivationScenario -AcceptEula -CloseOffice` : corrige automáticamente los errores relacionados con la activación, cierra Office y acepta el Contrato de licencia de usuario final (EULA) de este comando. Para ello se requieren privilegios de administrador
- `GetHelpCmd.exe -S OfficeActivationScenario -AcceptEula -CloseOffice -RemoveSCA` : desvincula el esquema de licencia de Office actualmente instalado de la activación de equipo compartido (SCA) y activa el producto por separado. Para ello se requieren privilegios de administrador
