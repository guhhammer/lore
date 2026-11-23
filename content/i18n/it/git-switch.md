# git switch

Passa ad altri rami. Richiede Git versione 2.23 o successiva.

## Usage
- `git switch nome_ramo` : passa ad un altro ramo esistente
- `git switch -c nome_ramo` : crea un nuovo ramo e passa a quel ramo
- `git switch -c nome_ramo commit` : crea un nuovo ramo a partire da un commit esistente e passa a quel ramo
- `git switch -` : torna al ramo precedente
- `git switch --recurse-submodules nome_ramo` : passa ad un ramo ed aggiorna tutti i moduli secondari associati
- `git switch -m nome_ramo` : passa ad un ramo e uniscilo automaticamente al ramo corrente, include le modifiche non committate
- `git switch -d tag|commit` : passa a un tag o a un commit
- `git switch -t nome_remoto/nome_ramo` : passa a un ramo di un altro remoto

## Links
- More information: https://git-scm.com/docs/git-switch
