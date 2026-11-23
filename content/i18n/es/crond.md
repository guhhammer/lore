# crond

Programa residente para ejecutar comandos programados desde archivos crontab.

## Usage
- `crond` : inicia el programa residente en segundo plano y comprueba los comandos programados
- `crond -n` : inicia el programa residente en primer plano y comprueba los comandos programados
- `crond -s` : envía la salida del trabajo desde el programa residente al registro del sistema
- `crond -p` : anula las limitaciones predeterminadas y acepta crontables personalizados
- `crond -P` : hereda la ruta del archivo crontab desde la configuración del entorno

## Links
- More information: https://manned.org/crond
