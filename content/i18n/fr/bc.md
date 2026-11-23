# bc

Un langage de calcul de précision arbitraire.

## Usage
- `bc` : démarre une session interactive
- `bc --mathlib` : démarre une session interactive avec la bibliothèque mathématique standard activée
- `echo '5 / 3' | bc` : calcule une expression
- `bc chemin/vers/le/script.bc` : exécute un script
- `echo 'scale = 10; 5 / 3' | bc` : calcule une expression avec l'échelle spécifiée
- `echo 's|c|a|l|e(1)' | bc --mathlib` : calcule une fonction sinus/cosinus/arctangente/logarithme naturel/exponentielle en utilisant `mathlib`

## Links
- More information: https://manned.org/bc
