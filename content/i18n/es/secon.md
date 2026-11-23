# secon

Obtiene el contexto de seguridad de SELinux de un archivo, un PID, el contexto de ejecución actual o una especificación de contexto.

## Usage
- `secon` : obtiene el contexto de seguridad del contexto de ejecución actual
- `secon -p 1` : obtiene el contexto de seguridad actual de un proceso
- `secon -f ruta/al/archivo_o_directorio` : obtiene el contexto de seguridad actual de un archivo, resolviendo todos los enlaces simbólicos intermedios
- `secon -L ruta/al/enlace_simbólico` : obtiene el contexto de seguridad actual del propio enlace simbólico (es decir, sin resolverlo)
- `secon system_u:system_r:container_t:s0:c899,c900` : analiza y explica una especificación de contexto

## Links
- More information: https://manned.org/secon
