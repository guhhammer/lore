# cp

Copiază fișiere și foldere.

## Usage
- `cp cale/către/fișier_sursă cale/către/fișier_destinație` : copiază un fișier într-o altă locație
- `cp cale/către/fișier_sursă cale/către/folder_destinație` : copiază un fișier într-un alt folder, păstrând numele fișierului
- `cp -r cale/către/folder_sursă cale/către/folder_destinație` : copiază recursiv conținutul unui folder într-o altă locație (dacă destinația există, folderul este copiat în interiorul acesteia)
- `cp -vr cale/către/folder_sursă cale/către/folder_destinație` : copiază recursiv un folder în mod detaliat (afișează fișierele pe măsură ce sunt copiate)
- `cp -t cale/către/folder_destinație cale/către/fișier1 cale/către/fișier2 ...` : copiază mai multe fișiere simultan într-un folder
- `cp -i *.ext cale/către/folder_destinație` : copiază toate fișierele cu o extensie specifică în altă locație, în mod interactiv (cere confirmare înainte de a suprascrie)
- `cp -L link cale/către/folder_destinație` : urmărește link-urile simbolice înainte de copiere
- `cp --parents sursă/cale/către/fișier cale/către/fișier_destinație` : folosește calea completă a fișierelor sursă, creând orice foldere intermediare lipsă în timpul copierii

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
