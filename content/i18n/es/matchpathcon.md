# matchpathcon

Consulta la configuración del contexto de seguridad persistente de SELinux para una ruta.

## Usage
- `matchpathcon /ruta/al/archivo` : consulta la configuración del contexto de seguridad persistente de una ruta absoluta
- `matchpathcon -m file|dir|pipe|chr_file|blk_file|lnk_file|sock_file /ruta/al/archivo` : restringe la consulta a la configuración de un tipo de archivo específico
- `matchpathcon -V /ruta/al/archivo` : verifica que los contextos de seguridad persistentes y actuales de una ruta coincidan

## Links
- More information: https://manned.org/matchpathcon.8
