# net

Utilidad del sistema para ver y modificar configuraciones relacionadas con la red.

## Usage
- `net start|stop servicio` : inicia o detiene un servicio de Windows de forma sincrónica
- `net use \\carpeta_compartida_smb /USER:usuario` : se asegura que un recurso compartido SMB esté disponible en la consola actual
- `net share` : muestra las carpetas actualmente compartidas a través de SMB
- `net session` : muestra quién está utilizando tus recursos compartidos SMB (ejecutar en consola elevada)
- `net localgroup "Administradores"` : muestra usuarios en un grupo de seguridad local
- `net localgroup "Administradores" usuario /add` : agrega un usuario al grupo de seguridad local (ejecutar en consola elevada)
- `net help subcomando` : muestra ayuda para un subcomando
- `net help` : muestra la ayuda
