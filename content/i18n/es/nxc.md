# nxc

Herramienta de enumeración y explotación de servicios de red. Algunos subcomandos como `smb` tienen su propia documentación de uso.

## Usage
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -L` : lista módulos disponibles para el protocolo especificado
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M nombre_del_módulo --options` : lista las opciones disponibles para el módulo especificado
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M nombre_del_módulo -o NOMBRE_OPCION=valor_opción` : especifica una opción para un módulo
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -h` : vea las opciones disponibles para el protocolo especificado

## Links
- More information: https://www.netexec.wiki/getting-started/selecting-and-using-a-protocol
