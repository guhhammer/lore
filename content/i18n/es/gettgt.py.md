# gettgt.py

Solicita un Ticket Granting Ticket (TGT). Parte del paquete Impacket.

## Usage
- `getTGT.py dominio/usuario:contraseña` : solicita un TGT utilizando una contraseña
- `getTGT.py -hashes LM_Hash:NT_Hash dominio/usuario` : solicita un TGT utilizando hash NTLM
- `getTGT.py -k -no-pass dominio/usuario` : utiliza la autenticación Kerberos (desde un ccache existente, no se necesita contraseña)
- `getTGT.py -aesKey clave_aes dominio/usuario` : solicita un TGT utilizando una clave AES (128 ó 256 bits)
- `getTGT.py -dc-ip ip_controlador_dominio dominio/usuario:contraseña` : especifica una IP de controlador de dominio
- `getTGT.py -service SPN dominio/usuario:contraseña` : solicita un tique de servicio directamente (AS-REQ) para un SPN específico

## Links
- More information: https://github.com/fortra/impacket
