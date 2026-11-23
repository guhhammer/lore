# dumpntlminfo.py

Realiza una autenticación NTLM en un host remoto sin credenciales y extrae la información filtrada en el mensaje NTLMSSP. Forma parte del paquete Impacket.

## Usage
- `DumpNTLMInfo.py destino` : extrae información NTLM del destino (SMB, puerto predeterminado 445)
- `DumpNTLMInfo.py -target-ip ip_destino destino` : extrae información NTLM utilizando una IP específica
- `DumpNTLMInfo.py -port puerto destino` : especifica un puerto personalizado
- `DumpNTLMInfo.py -protocol RPC -port 135 destino` : extrae información NTLM utilizando el protocolo RPC (puerto predeterminado 135)
- `DumpNTLMInfo.py -debug destino` : activa la salida de depuración

## Links
- More information: https://github.com/fortra/impacket
