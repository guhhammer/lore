# figlet

Genera encabezados usando caracteres ASCII desde la entrada del usuario.

## Usage
- `figlet texto_de_entrada` : genera el encabezado directamente introduciendo el texto
- `figlet -f ruta/al/archivo_de_fuente.flf texto_de_entrada` : usa un archivo de fuente personalizada
- `figlet -f archivo_de_fuente texto_de_entrada` : usa una fuente del directorio predeterminado (la extensión puede ser omitida)
- `comando | figlet` : redirige la salida de un comando hacia FIGlet
- `showfigfonts texto_opcional_para_mostrar` : muestra las fuentes de FIGlet disponibles
- `figlet -t -c texto_de_entrada` : utiliza el ancho total del terminal y centra el texto de entrada
- `figlet -W input_text` : muestra todos los caracteres utilizando todo su ancho para evitar traslapes

## Links
- More information: https://www.figlet.org/figlet-man.html
