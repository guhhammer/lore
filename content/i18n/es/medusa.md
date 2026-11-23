# medusa

Un forzador bruto de inicio de sesión modular y paralelo para una variedad de protocolos.

## Usage
- `medusa -d` : lista todos los módulos instalados
- `medusa -M ssh|http|web-form|postgres|ftp|mysql|... -q` : muestra ejemplo de uso de un módulo específico (usa `medusa -d` para listar todos los módulos instalados)
- `medusa -M ftp -h host -U ruta/al/archivo_de_usuario -P ruta/al/archivo_de_contraseña` : ejecuta fuerza bruta contra un servidor FTP utilizando un fichero que contiene nombres de usuario y un fichero que contiene contraseñas
- `medusa -M HTTP -h host -u usuario -p contraseña -m USER-AGENT:"agente"` : ejecuta un intento de inicio de sesión contra un servidor HTTP utilizando el nombre de usuario, la contraseña y el agente de usuario especificados
- `medusa -M mysql -h host -U ruta/al/archivo_de_usuario -p hash -m PASS:HASH` : ejecuta una fuerza bruta contra un servidor MySQL utilizando un fichero que contenga nombres de usuario y un hash
- `medusa -M smbnt -H ruta/al/archivo_de_hosts -C ruta/al/archivo_pwdump -u usuario -m PASS:HASH` : ejecuta una fuerza bruta contra una lista de servidores SMB utilizando un nombre de usuario y un archivo pwdump

## Links
- More information: https://manned.org/medusa
