# getst.py

Solicita un tique de servicio Kerberos (TGS). Forma parte del paquete Impacket.

## Usage
- `getST.py dominio/usuario:contraseña -spn servicio/destino` : solicita un tique de servicio para un SPN específico
- `getST.py -hashes LM_Hash:NT_Hash dominio/usuario -spn servicio/destino` : solicita un tique utilizando hash NTLM (pass-the-hash)
- `getST.py -no-pass -k dominio/usuario -spn servicio/destino` : solicita un tique utilizando un archivo ccache de Kerberos existente
- `getST.py -k -impersonate usuario_destino dominio/usuario -spn servicio/destino` : suplanta a otro usuario mediante S4U2Self (requiere derechos de delegación)
- `getST.py -force-forwardable -k dominio/usuario -spn servicio/destino` : fuerza que el tique sea reenviable (Bronze Bit)

## Links
- More information: https://github.com/fortra/impacket
