# filebrowser

Sencillo servidor web HTTP para gestionar archivos y directorios.

## Usage
- `filebrowser` : inicia una nueva instancia del servidor sirviendo el directorio actual
- `filebrowser -r ruta/al/directorio` : inicia una nueva instancia de servidor sirviendo un directorio raíz específico
- `filebrowser -a host -p puerto -r ruta/al/directorio` : inicia una instancia con una dirección de host (por defecto `127.0.0.1`) y un puerto (por defecto `8080`) diferentes
- `filebrowser -c ruta/al/archivo -d ruta/a/base_de_datos.db -r ruta/al/directorio` : inicia una instancia con un archivo de configuración especificado, almacenando la base de datos de la aplicación en una ubicación específica (por defecto es `filebrowser.db` en el directorio actual)
- `filebrowser --username usuario --password contraseña -r ruta/al/directorio` : configura un nombre de usuario y una contraseña diferentes para la primera cuenta (ambos por defecto `admin`) cuando configure una nueva instancia
- `filebrowser --img-processors 4 -r ruta/al/directorio` : configura la cantidad máxima de procesadores de imágenes utilizados al generar miniaturas (por defecto es `4`)
- `filebrowser --disable-exec --disable-thumbnails -r ruta/al/directorio` : desactiva las miniaturas de imágenes, así como la función Command Runner, que limita el acceso a los archivos de secuencia de comandos alojados para que no se ejecuten dentro de la aplicación
- `filebrowser --disable-preview-resize --disable-type-detection-by-header -r ruta/al/directorio` : deshabilita el cambio de tamaño de las vistas previas de imágenes, así como la detección de tipos de archivo mediante la lectura de sus cabeceras

## Links
- More information: https://filebrowser.org/cli/filebrowser.html
