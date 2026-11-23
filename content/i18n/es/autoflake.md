# autoflake

Una herramienta para eliminar importaciones y variables no utilizadas del código Python.

## Usage
- `autoflake --remove-unused-variables ruta/al/archivo.py` : elimina las variables no utilizadas de un archivo y muestra la diferencia
- `autoflake --remove-all-unused-imports ruta/al/archivo1.py ruta/al/archivo2.py ...` : elimina las importaciones no utilizadas de varios archivos y muestra las diferencias
- `autoflake --remove-unused-variables --in-place ruta/al/archivo.py` : elimina variables no utilizadas de un fichero, sobrescribiendo el fichero
- `autoflake --remove-unused-variables --in-place --recursive ruta/al/directorio` : elimina recursivamente las variables no utilizadas de todos los archivos de un directorio, sobrescribiendo cada archivo

## Links
- More information: https://github.com/PyCQA/autoflake#advanced-usage
