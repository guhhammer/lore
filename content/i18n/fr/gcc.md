# gcc

Préprocesse et compile des fichiers sources C et C++, pour ensuite les assembler et les lier entre eux.

## Usage
- `gcc chemin/vers/le/fichier1.c chemin/vers/le/fichier2.c ... -o chemin/vers/l'exécutable/à/produire` : compile plusieurs fichiers source en un exécutable
- `gcc chemin/vers/le/fichiersource.c -Wall -Og -o chemin/vers/l'exécutable/à/produire` : active les averissements et les symboles de débogage dans la sortie de commande
- `gcc chemin/vers/le/fichiersource.c -o chemin/vers/l'exécutable/à/produire -Ichemin/vers/les/fichiers/d'en-tête -Lchemin/vers/la/bibliothèque -lbibliothèque` : inclut les bibliothèques depuis un chemin différent de celui par défaut
- `gcc -S chemin/vers/le/fichiersource.c` : compile le code source en des instructions d'assemblage
- `gcc -c chemin/vers/le/fichiersource.c` : compile le code source en un fichier objet sans le lier

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
