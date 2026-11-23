# astyle

Indente, formate, et embelli du code source pour des languages de programmation comme C, C++, C# et Java. Pendant l'exécution, une copie du fichier original est créé avec un `.orig` suffixé au nom de fichier original.

## Usage
- `astyle fichier_source` : applique le style par défaut de 4 espaces pour l'indentation et pas de changement de formatage
- `astyle -A2 chemin/vers/fichier` : applique le style Java avec le style `attached`
- `astyle -A1 chemin/vers/fichier` : applique le style `allman`
- `astyle -s2..20 chemin/vers/fichier` : applique une indentation personnalisé avec des espaces. Choisi entre 2 et 20 espaces
- `astyle -t2..20 chemin/vers/fichier` : applique une indentation personnalisé avec des tabulations. Choisi entre 2 et 20 tabulations

## Links
- More information: https://manned.org/astyle
