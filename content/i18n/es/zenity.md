# zenity

Muestra diálogos desde guiones de la línea de comandos. Regresa los valores suministrados por el usuario o 1 si hay error.

## Usage
- `zenity --question` : muestra el diálogo predeterminado de pregunta
- `zenity --info --text "¡Hola!"` : muestra un diálogo de información que muestra el texto "¡Hola!"
- `zenity --forms --add-entry "Nombre" --add-password "Contraseña" --separator ";"` : muestra un formulario de nombre/contraseña y retorna los datos separados por ";"
- `zenity --file-selection --directory` : muestra un formulario de selección de archivos en el que el usuario sólo puede seleccionar directorios
- `(echo "#1"; sleep 1; echo "50"; echo "#2"; sleep 1; echo "100") | zenity --progress` : muestra una barra de progreso que actualiza su mensaje cada segundo y muestra un porcentaje de progreso

## Links
- More information: https://manned.org/zenity
