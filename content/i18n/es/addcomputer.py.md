# addcomputer.py

Añade una cuenta de ordenador al dominio. Parte del paquete Impacket.

## Usage
- `addcomputer.py -computer-name COMPUTER_NAME$ -computer-pass contraseña_computadora dominio/usuario:contraseña` : añade un ordenador con un nombre y una contraseña específicos
- `addcomputer.py -no-add -computer-name COMPUTER_NAME$ -computer-pass contraseña_computadora dominio/usuario:contraseña` : establece solo una nueva contraseña en un ordenador existente
- `addcomputer.py -delete -computer-name COMPUTER_NAME$ dominio/usuario:contraseña` : elimina una cuenta de ordenador existente
- `addcomputer.py -k -no-pass dominio/usuario@hostname` : añade un ordenador utilizando la autenticación Kerberos
- `addcomputer.py -method LDAPS -port 636 dominio/usuario:contraseña` : añade un ordenador a través de LDAPS (puerto 636) en lugar de SAMR (puerto 445)
- `addcomputer.py -dc-host hostname dominio/usuario:contraseña` : especifica el controlador de dominio exacto cuando existen varios DC

## Links
- More information: https://github.com/fortra/impacket
