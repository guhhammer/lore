# mysql

Herramienta de línea de comandos para gestionar bases de datos MySQL.

## Usage
- `mysql nombre_base_de_datos` : conecta a una base de datos
- `mysql -u usuario -p nombre_base_de_datos` : conecta a una base de datos con el usuario `usuario` y se le pedirá la contraseña
- `mysql -h maquina_remota nombre_base_de_datos` : conecta a una base de datos en otra máquina
- `mysql -S ruta/al/socket.sock` : conecta a una base de datos a través de un socket unix
- `mysql -e "source archivo.sql" nombre_base_de_datos` : ejecuta comandos SQL contenidos en un script
- `mysql <ruta/al/backup.sql -u usuario -p nombre_base_de_datos` : restaura una base de datos a partir de una copia de seguridad creada con `mysqldump` (y se le pedirá la contraseña al usuario)
- `mysql <ruta/al/backup.sql -u usuario -p` : restaura todas las bases de datos en una copia de seguridad (y se le pedirá la contraseña al usuario)

## Links
- More information: https://manned.org/mysql
