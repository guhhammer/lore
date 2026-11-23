# perl

Interpréteur du langage Perl (version 5).

## Usage
- `perl fichier.pl` : exécuter le code contenu dans un fichier
- `perl -c fichier.pl` : vérifier la syntaxe sans exécuter le programme
- `perl -e expression` : exécuter une expression Perl
- `perl -d fichier.pl` : lancer le programme avec le debugger Perl
- `perl -p -i -e 's/recherche/remplacement' fichier` : itérer sur toutes les lignes d'un fichier, en les modifiant sur place en utilisant une expression de recherche et de remplacement
- `perl -p -i'.ancien' -e 's/recherche/remplacement/g' fichier` : lancer une expression de recherche et remplacement sur un fichier, en sauvegardant le fichier original avec une autre extension
- `perl -p0e 's/recherche/remplacement/g' fichier_entrée >fichier_sortie` : lancer une expression de recherche et remplacement sur un fichier, en sauvegardant le résultat dans un autre fichier
- `cat fichier_entrée | perl -nle 'if (/regex/) { print "$1"; last;}'` : lancer une `regex` sur `stdin`, en affichant le premier groupe capturé pour chaque ligne

## Links
- More information: https://perldoc.perl.org/perl
