# secretsdump.py

Vuelca hashes NTLM, contraseñas en texto plano y credenciales de dominio de sistemas Windows remotos. Parte de la suite Impacket.

## Usage
- `secretsdump.py dominio/usuario:contraseña@objetivo` : vuelca credenciales de una máquina Windows utilizando un nombre de usuario y una contraseña
- `secretsdump.py -hashes LM_Hash:NT_Hash dominio/usuario@destino` : vuelca hashes de una máquina utilizando autenticación pass-the-hash
- `secretsdump.py -just-dc dominio/usuario:contraseña@destino` : vuelca credenciales del archivo NTDS.dit de Active Directory
- `secretsdump.py -sam ruta/a/SAM -system ruta/a/SYSTEM` : extrae credenciales de una base de datos SAM local utilizando archivos hives del registro
- `secretsdump.py -no-pass dominio/usuario@destino` : vuelca hashes de una máquina sin proporcionar una contraseña (si existe una sesión de autenticación válida, por ejemplo, a través de Kerberos o NTLM SSO)

## Links
- More information: https://github.com/fortra/impacket
