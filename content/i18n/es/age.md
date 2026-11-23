# age

Una herramienta de cifrado de archivos sencilla, moderna y segura.

## Usage
- `age -p -o ruta/al/archivo_cifrado.age ruta/al/archivo_no_cifrado` : genera un archivo cifrado que se puede descifrar con una frase de contraseña
- `age -r clave_publica -o ruta/al/archivo_cifrado.age ruta/al/archivo_no_cifrado` : cifra un archivo con una o más claves públicas introducidas como literales (repetir `--recipient` para varias claves)
- `age -R ruta/al/archivo_destinatarios.txt -o ruta/al/archivo_cifrado.age ruta/al/archivo_no_cifrado` : cifra un archivo con las claves públicas especificadas en un archivo de destinatarios (una por línea)
- `age -d -o ruta/al/archivo_descifrado ruta/al/archivo_cifrado.age` : descifra un archivo con una frase de contraseña
- `age -d -i ruta/al/archivo_clave_privada -o ruta/al/archivo_descifrado ruta/al/archivo_cifrado.age` : descifra un archivo con un archivo de clave privada

## Links
- More information: https://github.com/FiloSottile/age#usage
