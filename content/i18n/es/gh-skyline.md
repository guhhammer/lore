# gh skyline

Genera un modelo 3D de tu historial de contribuciones a GitHub. Por defecto, creará un archivo `{usuario}-{año}-github-skyline.stl` en el directorio actual.

## Usage
- `gh skyline` : genera un archivo STL de la línea del horizonte para el año actual y el usuario autenticado
- `gh skyline -u usuario -y año` : genera una línea del horizonte para un usuario y un año y específicos
- `gh skyline -u usuario -y primer_año-último_año` : genera una línea del horizonte para un intervalo de años
- `gh skyline -u usuario -f` : genera una línea del horizonte completa f (desde el año de alta del usuario hasta el año actual)
- `gh skyline -d` : habilita el registro de depuración
- `gh skyline -o ruta/al/archivo_salida.stl` : genera una línea del horizonte y especifica la ruta del archivo de salida
- `gh skyline -u usuario -w` : abre el perfil de GitHub de un usuario específico
- `gh skyline -h` : muestra la ayuda

## Links
- More information: https://github.com/github/gh-skyline
