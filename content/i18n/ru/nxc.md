# nxc

Инструмент для перечисления и эксплуатации сетевых сервисов. Некоторые подкоманды, такие как `smb`, имеют собственную документацию.

## Usage
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -L` : вывести список доступных модулей для указанного протокола
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M имя_модуля --options` : вывести список параметров для указанного модуля
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M имя_модуля -o НАЗВАНИЕ_ОПЦИИ=значение_опции` : указать oпцию для модуля
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -h` : показать параметры для указанного протокола

## Links
- More information: https://www.netexec.wiki/getting-started/selecting-and-using-a-protocol
