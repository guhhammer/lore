# nxc ssh

Prueba y explota servidores SSH.

## Usage
- `nxc ssh 192.168.178.2 -u ruta/a/nombres_de_usuario.txt -p contraseña` : pulveriza la contraseña especificada contra una lista de nombres de usuario en el objetivo especificado
- `nxc ssh 192.168.178.2 -u ruta/a/nombres_de_usuario.txt -p ruta/a/contraseñas.txt` : busca credenciales válidas probando todas las combinaciones de nombres de usuario y contraseñas de las listas especificadas
- `nxc ssh 192.186.178.2 -u ruta/a/nombres_de_usuario.txt -p contraseña --key-file ruta/a/id_rsa` : utiliza la clave privada especificada para la autenticación, utilizando la contraseña suministrada como frase de contraseña de la clave
- `nxc ssh 192.168.178.0/24 -u usuario -p contraseña` : prueba una combinación de nombres de usuario y contraseñas en una serie de objetivos
- `nxc ssh 192.168.178.2 -u usuario -p ruta/a/contraseñas.txt --sudo-check` : comprueba los privilegios `sudo` en un inicio de sesión correcto

## Links
- More information: https://www.netexec.wiki/ssh-protocol/password-spraying
