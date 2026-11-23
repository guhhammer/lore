# cpdf

Interfaz de línea de comandos para manipular documentos PDF existentes de diferentes maneras.

## Usage
- `cpdf ruta/al/documento_fuente.pdf 1-3,6 -o ruta/al/documento_objetivo.pdf` : selecciona las páginas 1, 2, 3 y 6 del documento fuente y las agrega en el documento objetivo
- `cpdf -merge ruta/al/documento_fuente_uno.pdf ruta/al/documento_fuente_dos.pdf -o ruta/al/documento_objetivo.pdf` : fusiona dos documentos en uno nuevo
- `cpdf -list-bookmarks ruta/al/documento.pdf` : muestra los marcadores del documento
- `cpdf -split ruta/al/documento.pdf -o ruta/al/fragmento%%%.pdf -chunk 10` : divide un documento en trozos de diez páginas, escribiendo `fragmento001.pdf`, `fragmento002.pdf`, etc
- `cpdf -encrypt 128bit fred joe ruta/al/documento_fuente.pdf -o ruta/al/documento_encriptado.pdf` : encripta un documento utilizando encriptado 128bit y establece `fred` como la contraseña del propietario y `joe` como la contraseña de usuario
- `cpdf -decrypt ruta/al/documento_encriptado.pdf owner=fred -o ruta/al/documento_desencriptado.pdf` : desencripta un documento utilizando la contraseña del propietario (`fred`)
- `cpdf -list-annotations ruta/al/documento.pdf` : muestra las anotaciones de un documento
- `cpdf -set-metadata ruta/de/los/metadatos.xml ruta/al/documento_fuente.pdf -o ruta/al/documento_objetivo.pdf` : crea un nuevo documento, con metadatos, a partir de uno que ya existe

## Links
- More information: https://www.coherentpdf.com/cpdfmanual/cpdfmanual.html
