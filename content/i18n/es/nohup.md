# nohup

Permite que un proceso siga ejecutándose cuando se cierra el terminal.

## Usage
- `nohup comando argumento1 argumento2 ...` : ejecuta un proceso que puede seguir ejecutándose tras el cierre del terminal
- `nohup comando argumento1 argumento2 ... &` : ejecuta `nohup` en modo de segundo plano
- `nohup ruta/al/script.sh &` : ejecuta un script de intérprete de comandos que puede seguir ejecutándose tras el cierre del terminal
- `nohup comando argumento1 argumento2 ... >ruta/al/archivo_de_salida &` : ejecuta un proceso y escribe la salida en un archivo específico
