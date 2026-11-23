# grep

Recherche des motifs dans un texte. Supporte des motifs simples et des `regex`.

## Usage
- `grep "chaîne_recherchée" chemin/vers/fichier` : recherche une chaîne de caractères précise
- `grep -F "chaîne_recherchée" chemin/vers/fichier` : recherche en ignorant la casse
- `grep -r -n "chaîne_recherchée" .` : recherche récursivement (en ignorant les fichiers non-texte) dans le dossier courant une chaîne de caractères précise
- `grep -Ei regex chemin/vers/fichier` : utilise des `regex` étendues (supporte `?`, `+`, `{}`, `()` et `|`)
- `grep --context|--before-context|--after-context 3 "chaîne_recherchée" chemin/vers/fichier` : affiche 3 lignes de Contexte, avant (Before), ou Après chaque concordance
- `grep -Hn --color=always "chaîne_recherchée" chemin/vers/fichier` : affiche le nom du fichier avec la ligne correspondante pour chaque concordance
- `cat chemin/vers/fichier | grep "chaîne_recherchée"` : utilise l'entrée standard au lieu d'un fichier

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
