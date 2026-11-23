# git merge-index

Rulează un program de fuziune pe fișierele care necesită fuziune.

## Usage
- `git merge-index git-merge-one-file -a` : fuzionează toate fișierele care necesită rezolvare folosind ajutorul standard
- `git merge-index git-merge-one-file -- cale/către/fișier` : fuzionează un fișier specific
- `git merge-index -o git-merge-one-file -- cale/către/fișier1 cale/către/fișier2 ...` : fuzionează mai multe fișiere, continuând în caz de erori
- `git merge-index -q merge-program -a` : fuzionează în tăcere toate fișierele cu un program personalizat
- `git merge-index cat -- cale/către/fișier` : inspectează intrările de fuziune pentru un fișier folosind `cat`

## Links
- More information: https://git-scm.com/docs/git-merge-index
