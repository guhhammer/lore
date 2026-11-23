# top

Mostra informació dinàmica en temps real sobre processos executant-se.

## Usage
- `top` : inicia top
- `top -i` : no mostra cap procés inactiu o zombie
- `top -u usuari` : mostra només processos pertanyents a un usari donat
- `top -o nom_columna` : ordena processos per una columna
- `top -Hp id_procés` : mostra els fils individuals d'un procés donat
- `top -p $(pgrep -d ',' nom_procés)` : mostra només els processos amb un(s) PID(s) donat(s), separats per comes. (Normalment no es coneix el PID amb antelació. Aquest exemple l'obté del nom del procés)
- `<?>` : obté ajuda sobre els commandaments interactius

## Links
- More information: https://manned.org/top
