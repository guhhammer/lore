# mqtt_check.py

Una simple utilidad para probar y validar las credenciales de inicio de sesión MQTT. Parte de la suite Impacket.

## Usage
- `mqtt_check.py dominio/usuario:contraseña@nombre_del_destino` : comprueba las credenciales de inicio de sesión MQTT para un objetivo (nombre del host del broker MQTT)
- `mqtt_check.py -client-id id_cliente dominio/usuario:contraseña@nombre_del_destino` : especifique un ID de cliente personalizado para la autenticación
- `mqtt_check.py -ssl dominio/usuario:contraseña@nombre_del_destino` : habilita SSL para la conexión
- `mqtt_check.py -port puerto dominio/usuario:contraseña@nombre_del_destino` : se conecta a un puerto específico (por defecto es 1883)
- `mqtt_check.py -debug dominio/usuario:contraseña@nombre_del_destino` : habilita salida de depuración
- `mqtt_check.py --help` : muestra la ayuda

## Links
- More information: https://github.com/fortra/impacket
