# rsactftool.py

Herramienta de ataque RSA para desafíos CTF - recupera claves privadas a partir de claves públicas débiles y/o descifra datos.

## Usage
- `RsaCtfTool.py --publickey ruta/a/clave.pub --private` : recupera una clave privada de un archivo de clave pública
- `RsaCtfTool.py --publickey ruta/a/clave.pub --decryptfile ruta/al/archivo_cifrado` : descifra un fichero utilizando una clave pública
- `RsaCtfTool.py --publickey ruta/a/clave.pub --decrypt "texto_cifrado"` : descifra una cadena de texto cifrado específica
- `RsaCtfTool.py --dumpkey --key ruta/a/clave.pub` : vuelca los componentes de una clave RSA (por ejemplo, módulo, exponente) desde un archivo de claves
- `RsaCtfTool.py --publickey ruta/a/clave.pub --private --attack fermat` : ejecuta un ataque específico (por ejemplo, factorización de Fermat) para recuperar la clave privada
- `RsaCtfTool.py --createpub -n módulo -e exponente` : genera una clave pública a partir del módulo (n) y el exponente (e)
- `RsaCtfTool.py --publickey ruta/a/clave.pub --private --attack all` : intenta todos los ataques disponibles para recuperar la clave privada

## Links
- More information: https://github.com/RsaCtfTool/RsaCtfTool#usage
