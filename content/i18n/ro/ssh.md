# ssh

Secure Shell este un protocol folosit pentru a te conecta în siguranță la sisteme la distanță. Poate fi folosit pentru autentificare sau pentru a rula comenzi pe un server la distanță.

## Usage
- `ssh nume_utilizator@server_distant` : conectare la un server la distanță
- `ssh nume_utilizator@server_distant -i cale/către/fișier_cheie` : conectare la un server la distanță cu o identitate specifică (cheie privată)
- `ssh nume_utilizator@10.0.0.1 -p 2222` : conectare la un server la distanță cu IP-ul `10.0.0.1` folosind un port specific (Notă: `10.0.0.1` poate fi scurtat la `10.1`)
- `ssh nume_utilizator@server_distant -t comandă argumente_comandă` : rulează o comandă pe un server la distanță cu alocare tty pentru a permite interacțiunea cu comanda
- `ssh nume_utilizator@server_distant -D 1080` : tunelare SSH: redirecționare Dinamică de port (proxy SOCKS pe `localhost:1080`)
- `ssh nume_utilizator@server_distant -L 9999:example.org:80 -N -T` : tunelare SSH: redirecționează un port specific (`localhost:9999` către `example.org:80`) dezactivând alocarea de pseudo-Tty și execuțiNa comenzilor la distanță
- `ssh nume_utilizator@server_distant -J nume_utilizator@jump_host` : salt SSH (Jumping): conectare printr-un jumphost către un server la distanță (mai multe salturi pot fi specificate separate prin virgulă)
- `<Enter><~><.>` : închide o sesiune blocată

## Links
- More information: https://man.openbsd.org/ssh
