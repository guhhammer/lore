# w32tm

Consulta y controla el servicio de sincronización de tiempo w32time.

## Usage
- `w32tm /query /status /verbose` : muestra el estado actual de la sincronización de tiempo
- `w32tm /stripchart /computer:servidor_de_tiempo` : muestra un gráfico de desfase de tiempo contra un servidor horario
- `w32tm /stripchart /packetinfo /samples:1 /computer:servidor_de_tiempo` : muestra una respuesta NTP de un servidor horario
- `w32tm /query /peers` : muestra el estado de los servidores de tiempo usados actualmente
- `w32tm /query /configuration` : muestra la configuración del servicio w32time (ejecutar en consola con privilegios elevados)
- `w32tm /resync /force` : fuerza la resincronización de tiempo inmediatamente (ejecutar en consola con privilegios elevados)
- `w32tm /debug /enable /file:ruta\a\debug.log /size:10000000 /entries:0-300` : escribe los registros de depuración de w32time en un archivo (ejecutar en consola con privilegios elevados)
