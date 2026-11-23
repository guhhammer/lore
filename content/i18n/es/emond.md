# emond

Servicio de Monitor de Eventos que acepta eventos de varios servicios, los ejecuta a través de un simple motor de reglas, y toma acciones. Las acciones pueden ejecutar comandos, enviar correos electrónicos o mensajes SMS.

## Usage
- `emond` : inicia el daemon
- `emond -r ruta/al/archivo_o_directorio` : especifica las reglas que emond debe procesar indicando una ruta a un archivo o directorio
- `emond -c ruta/al/archivo_de_configuración` : utiliza un archivo de configuración específico

## Links
- More information: https://manpagez.com/man/8/emond/
