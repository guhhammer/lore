# olevba

Analiza archivos OLE y OpenXML (p. ej., DOC, XLS, PPT, etc.) para extraer macros VBA, desofuscar y analizar código malicioso. Parte de la suite `python-oletools`.

## Usage
- `olevba ruta/al/archivo` : analiza un archivo, mostrando tanto el código de la macro como los resultados del análisis
- `olevba -r ruta/al/directorio` : analiza recursivamente todos los archivos compatibles de un directorio
- `olevba -p contraseña ruta/al/archivo_encriptado` : proporciona una contraseña para los archivos cifrados de Microsoft Office (puede repetirse)
- `olevba -a ruta/al/archivo` : muestra solo los resultados del análisis, sin mostrar el código fuente de la macro
- `olevba -c ruta/al/archivo` : muestra solo el código fuente de la macro
- `olevba --decode ruta/al/archivo` : muestra cadenas ofuscadas y su contenido decodificado

## Links
- More information: https://github.com/decalage2/oletools
