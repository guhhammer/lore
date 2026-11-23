# age

Strumento semplice, moderno e sicuro per la crittografia di file.

## Usage
- `age -p -o percorso/file_crittografato.age percorso/file_non_crittografato` : crea un file crittografato che può essere decrittato con una passphrase
- `age -r chiave_pubblica -o percorso/file_crittografato.age percorso/file_non_crittografato` : crittografa un file con una o più chiavi pubbliche inserite come literali (ripeti il flag `--recipient` per specificare più chiavi pubbliche)
- `age -R percorso/file_destinatari.txt -o percorso/file_crittografato.age percorso/file_non_crittografato` : crittografa un file per uno o più destinatari con le loro chiavi pubbliche specificate in un file (una per riga)
- `age -d -o percorso/file_decrittografato percorso/file_crittografato.age` : decrittografa un file con una passphrase
- `age -d -i percorso/file_chiave_privata -o percorso/file_decrittografato percorso/file_crittografato.age` : decrittografa un file con un file di chiave privata

## Links
- More information: https://github.com/FiloSottile/age#usage
