# crontab

Programa trabajos recurrentes (cron jobs) para ejecutarse a intervalos de tiempo para el usuario actual.

## Usage
- `crontab -e` : edita el archivo crontab para el usuario actual
- `sudo crontab -e -u usuario` : edita el archivo crontab para un usuario específico
- `crontab ruta/al/archivo` : reemplaza el crontab actual con el contenido del archivo dado
- `crontab -l` : muestra una lista de cron jobs existentes para el usuario actual
- `crontab -r` : elimina todos los cron jobs para el usuario actual
- `0 10 * * * comando_a_ejecutar` : ejemplo de entrada contrab que ejecuta un comando a las 10:00 cada día (* significa cualquier valor)
- `*/10 * * * * comando_a_ejecutar` : ejemplo de entrada crontab, que ejecuta un comando cada 10 minutos
- `30 2 * * Fri //ruta/a/script.sh` : ejemplo de entrada crontab, que ejecuta un script a las 02:30 cada viernes

## Links
- More information: https://manned.org/crontab
