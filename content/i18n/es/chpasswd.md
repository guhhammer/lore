# chpasswd

Cambia las contraseñas de varios usuarios utilizando `stdin`.

## Usage
- `printf "usuario:nueva_contraseña" | sudo chpasswd` : cambia la contraseña de un usuario específico
- `printf "nombre_usuario_1:nueva_contraseña_1\nnombre_usuario_2:nueva_contraseña_2" | sudo chpasswd` : cambia las contraseñas de varios usuarios (el texto ingresado no debe contener espacios)
- `printf "usuario:nueva_contraseña_cifrada" | sudo chpasswd -e` : cambia la contraseña de un usuario específico y es especificada en forma cifrada
- `printf "usuario:nueva_contraseña" | sudo chpasswd -c NONE|DES|MD5|SHA256|SHA512` : cambia la contraseña de un usuario específico y utiliza un cifrado específico para la contraseña almacenada

## Links
- More information: https://manned.org/chpasswd
