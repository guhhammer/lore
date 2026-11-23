# vim

Vim (Vi IMproved), un éditeur de texte en ligne de commandes, fournit plusieurs modes pour différentes manipulations de texte. Pressez `<i>` pour passer en mode édition. `<Esc>` revient au mode normal, qui ne permet pas l insertion de code.

## Usage
- `vim chemin/vers/fichier` : ouvre un fichier
- `vim +numero_ligne chemin/vers/fichier` : ouvre un fichier à une ligne spécifiée
- `<:>help<Enter>` : consulte le manuel utilisateur
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : sauvegarde et ferme
- `<Esc><u>` : annule la dernière opération
- `</>motif_recherché<Entrée>` : recherche un motif dans un fichier (appuyer `<n>`/`<N>` pour aller à la prochaine / précédente occurrence)
- `<:>%s/regex/remplacement/g<Entrée>` : effectuer une substitution par `regex` dans tout le fichier
- `<:>set nu<Entrée>` : affiche les numéros de ligne

## Links
- More information: https://www.vim.org/
