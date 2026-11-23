# agetty

Alternativa a `getty`: Abre un puerto `tty`, pide un nombre de usuario, e invoca el comando `/bin/login`. Normalmente es invocado por `init`. Nota: la tasa de baudios es la velocidad de transferencia de datos entre una terminal y un dispositivo a través de una conexión serie.

## Usage
- `agetty tty 115200` : conecta `stdin` a un puerto (relativo a `/dev`) y especifica opcionalmente una tasa de baudios (cuyo valor predeterminado es 9600)
- `agetty -t tiempo_de_espera_en_segundos -` : asume que `stdin` ya está conectado a una `tty` y establece un tiempo de espera para el inicio de sesión
- `agetty -8 - variable_term` : asume que `tty` es de [8]-bits, sobreescribiendo la variable de entorno `$TERM` establecida por `init`
- `agetty -n -l programa_de_inicio_de_sesión tty` : omite el inicio de sesión e invoca, como superusuario, otro programa de inicio de sesión en lugar de `/bin/login`
- `agetty -i -` : escribe el mensaje de inicio de sesión sin mostrar el contenido del archivo de pre-inicio de sesión (`/etc/issue` por predeterminado)
- `agetty -r /ruta/a/raíz_directorio -H host_falso -` : cambia el directorio raíz y escribe un host falso en el archivo `utmp`

## Links
- More information: https://manned.org/agetty
