# getadusers.py

Recupera una lista de usuarios de Active Directory, incluyendo atributos como la fecha y hora del último inicio de sesión y el correo electrónico. Parte del paquete Impacket.

## Usage
- `GetADUsers.py -all -dc-ip ip_controlador_dominio dominio/usuario:contraseña` : enumera todos los usuarios de Active Directory y sus atributos
- `GetADUsers.py -user usuario -dc-ip ip_controlador_dominio dominio/usuario:contraseña` : recupera información solo para un usuario específico
- `GetADUsers.py -all -dc-ip ip_controlador_dominio -hashes LM_Hash:NT_Hash dominio/usuario` : extrae los detalles del usuario utilizando la autenticación pass-the-hash
- `GetADUsers.py -all -dc-ip ip_controlador_dominio dominio/usuario:contraseña >ruta/a/salida.txt` : guarda la salida en un archivo

## Links
- More information: https://github.com/fortra/impacket
