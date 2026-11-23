# asciinema

Enregistre et rejoue les sessions de terminal, et également partageable sur asciinema.org.

## Usage
- `asciinema a` : associe l’installation locale de `asciinema` avec un compte asciinema.org
- `asciinema r chemin/vers/enregistrement.cast` : crée un nouvel enregistrement et l'enregistre dans un fichier local (finir avec `<Ctrl d>` ou taper `exit`)
- `asciinema p chemin/vers/enregistrement.cast` : rejoue un enregistrement depuis un fichier local
- `asciinema p https://asciinema.org/a/id_d_enregistrement` : rejoue un enregistrement depuis <https://asciinema.org>
- `asciinema r -i 2.5` : crée un nouvel enregistrement, en limitant le temps d’inactivité au maximum à 2.5 secondes
- `asciinema ca chemin/vers/enregistrement.cast` : affiche la sortie complète d'un enregistrement local
- `asciinema u chemin/vers/enregistrement.cast` : envoie un enregistrement local vers asciinema.org
- `asciinema st --local` : diffuse le terminal actuel sur une page web locale

## Links
- More information: https://docs.asciinema.org/manual/cli/
