# adscript

Compilateur pour fichiers Adscript.

## Usage
- `adscript --output chemin/vers/fichier.o chemin/vers/fichier_source.adscript` : compile un fichier vers un fichier objet
- `adscript --executable --output chemin/vers/fichier chemin/vers/fichier_source.adscript` : compile et lie un fichier vers un exécutable autonome
- `adscript --llvm-ir --output chemin/vers/fichier.ll chemin/vers/fichier_source.adscript` : compile un fichier vers LLVM IR à la place du code machine natif
- `adscript --target-triple i386-linux-elf --output chemin/vers/fichier.o chemin/vers/fichier_source.adscript` : fait une compilation croisée d'un fichier vers un fichier objet pour une architecture CPU ou un système d'exploitation distant

## Links
- More information: https://github.com/Amplus2/Adscript
