# getuserspns.py

Recupera los Service Principal Names (SPNs) asociados a las cuentas de usuario de Active Directory. Parte de la suite Impacket.

## Usage
- `GetUserSPNs.py dominio/usuario:contraseña -dc-ip dominio_controlador_ip` : enumera las cuentas de usuario con un SPN y solicita sus tickets TGS de Kerberos
- `GetUserSPNs.py dominio/usuario -hashes LM_Hash:NT_Hash -dc-ip dominio_controlador_ip` : usa autenticación pass-the-hash
- `GetUserSPNs.py dominio/usuario:contraseña -dc-ip dominio_controlador_ip -outputfile ruta/al/archivo_salida` : guarda el resultado en un archivo
- `GetUserSPNs.py dominio/usuario:contraseña -dc-ip dominio_controlador_ip -request` : pide solo tickets TGS
- `GetUserSPNs.py dominio/usuario -dc-ip dominio_controlador_ip -hashes LM_Hash:NT_Hash -request` : solicita solo tickets TGS utilizando autenticación pass-the-hash

## Links
- More information: https://github.com/fortra/impacket
