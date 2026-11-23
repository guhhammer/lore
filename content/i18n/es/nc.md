# nc

Redirige datos de entrada o salida a un flujo de red a través de esta versátil herramienta.

## Usage
- `nc <nombre_de_archivo -l -p puerto` : inicia un escuchador en un puerto TCP y le envía un archivo
- `nc host puerto >nombre_de_archivo_por_recibir` : conecta a un escuchador en un puerto y recibe un archivo de él
- `nc -v -z -w tiempo_de_espera_en_segundos host puerto_inicial-puerto_final` : escanea los puertos TCP abiertos en un host
- `nc -l -p puerto -e ejecutable_del_intérprete` : inicia un escuchador en un puerto TCP y provee de acceso a tu intérprete de comandos local a la parte conectada (esto es peligroso y podría ser explotado)
- `nc host puerto -e ejecutable_del_intérprete` : conecta a un escuchador y provee de acceso a tu intérprete de comandos local a una parte remota (esto es peligroso y podría ser explotado)
- `nc -l -p puerto_local | nc host puerto_remoto` : actúa como un proxy y envía información de un puerto TCP local a un host remoto
- `echo -e "GET / HTTP/1.1\nHost: host\n\n" | nc host 80` : envía una petición HTTP GET

## Links
- More information: https://manned.org/nc
