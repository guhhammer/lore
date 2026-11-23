# look

Affiche les lignes commençant par un préfixe dans un fichier trié.

## Usage
- `look préfixe chemin/vers/fichier` : recherche les lignes commençant par un préfixe spécifique dans un fichier spécifique
- `look -f -d préfixe chemin/vers/fichier` : effectue une recherche insensible à la casse en utilisant uniquement les caractères alphanumériques
- `look -t ,` : spécifie un caractère de terminaison de chaîne (espace par défaut)
- `look préfixe` : recherche dans `/usr/share/dict/words` (`--ignore-case` et `--alphanum` sont activées par défaut)

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=look
