# nxc

Tool voor netwerkservice-enumeratie en -exploitatie. Sommige subcommando's zoals `smb` hebben hun eigen documentatie.

## Usage
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -L` : toon een lijst van beschikbare modules voor het opgegeven protocol
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M module_naam --options` : toon de opties die beschikbaar zijn voor de opgegeven module
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M module_naam -o OPTIE_NAAM=optie_waarde` : geef een optie op voor een module
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -h` : bekijk de opties die beschikbaar zijn voor het opgegeven protocol

## Links
- More information: https://www.netexec.wiki/getting-started/selecting-and-using-a-protocol
