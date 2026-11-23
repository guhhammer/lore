# gpg

GNU Privacy Guard.

## Usage
- `gpg --full-gen-key` : cria uma chave GPG pública e privada interativamente
- `gpg --clearsign doc.txt` : assina doc.txt sem criptografia (cria um arquivo de saída `doc.txt.asc`)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : criptografa e assina `doc.txt` para alice@example.com e bob@example.com (cria um arquivo de saída `doc.txt.gpg`)
- `gpg -c doc.txt` : criptografa `doc.txt` apenas com uma senha simétrica (cria um arquivo de saída `doc.txt.gpg`)
- `gpg -d doc.txt.gpg` : descriptografa `doc.txt.gpg` (envia saída para `stdout`)
- `gpg --import public.gpg` : importa uma chave pública
- `gpg --export -a alice@example.com` : exporta a chave pública da alice@example.com (envia saída para `stdout`)
- `gpg --export-secret-keys -a alice@example.com` : exporta chave privada da alice@example.com (envia saída para `stdout`)

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
