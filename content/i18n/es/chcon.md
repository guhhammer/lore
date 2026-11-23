# chcon

Cambia el contexto de seguridad de SELinux de uno o varios archivos o directorios.

## Usage
- `ls -lZ ruta/al/archivo` : vea el contexto de seguridad de un archivo
- `chcon --reference archivo_de_referencia archivo_de_destino` : cambia el contexto de seguridad de un archivo de destino utilizando un archivo de referencia
- `chcon usuario:rol:tipo:rango/nivel nombre_archivo` : cambia el contexto de seguridad completo de SELinux de un archivo
- `chcon -u usuario nombre_del_archivo` : cambia únicamente la parte de usuario del contexto de seguridad de SELinux
- `chcon -r rol nombre_del_archivo` : cambia únicamente la parte de rol del contexto de seguridad de SELinux
- `chcon -t tipo nombre_del_archivo` : cambia únicamente la parte de tipo del contexto de seguridad de SELinux
- `chcon -l rango/nivel nombre_del_archivo` : cambia únicamente la parte de rango/nivel del contexto de seguridad de SELinux
