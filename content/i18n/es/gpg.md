# gpg

GNU Privacy Guard, una herramienta de cifrado y firma OpenPGP.

## Usage
- `gpg --full-gen-key` : crea una clave pública y privada GPG de forma interactiva
- `gpg -k` : lista todas las claves del llavero público
- `gpg --clearsign doc.txt` : firma `doc.txt` sin cifrar (escribe el resultado en `doc.txt.asc`)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : cifra y firma `doc.txt` para `alice@example.com` y `bob@example.com` (salida a `doc.txt.gpg`)
- `gpg -c doc.txt` : cifra `doc.txt` con solo una frase de contraseña (salida a `doc.txt.gpg`)
- `gpg -d doc.txt.gpg` : descifra `doc.txt.gpg` (salida a `stdout`)
- `gpg --import public.gpg` : importa una clave pública
- `gpg --export|--export-secret-keys -a alice@example.com` : exporta la clave pública/privada para `alice@example.com` (salida a `stdout`)

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
