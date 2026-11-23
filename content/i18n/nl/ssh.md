# ssh

Secure Shell is een protocol waarmee op een veilige manier ingelogd kan worden op externe systemen. Het kan gebruikt worden voor het loggen of uitvoeren van opdrachten op een externe server.

## Usage
- `ssh gebruikersnaam@externe_host` : verbind met een externe server
- `ssh gebruikersnaam@externe_host -i pad/naar/sleutel_bestand` : verbind met een externe server met een specifieke identiteit (privésleutel)
- `ssh gebruikersnaam@10.0.0.1 -p 2222` : verbind met een externe server met IP `10.0.0.1` en gebruik een specifieke poort (Opmerking: `10.0.0.1` kan worden afgekort tot `10.1`)
- `ssh gebruikersnaam@externe_host -t commando opdrachtargumenten` : voer een opdracht uit op een externe server met een tty-toewijzing die interactie met de externe opdracht toestaat
- `ssh gebruikersnaam@externe_host -D 1080` : SSH-tunneling: Dynamische poortdoorsturing (SOCKS-proxy op `localhost:1080`)
- `ssh gebruikersnaam@externe_host -L 9999:example.org:80 -N -T` : SSH-tunneling: Stuur een specifieke poort door (`localhost:9999` naar `example.org:80`) en schakel pseudo-Tty toewijzing en uitvoeriNg van externe opdrachten uit
- `ssh gebruikersnaam@externe_host -J gebruikersnaam@jump_host` : SSH Jumping: Verbind door een jumphost met een externe server (Meerdere jump hops mogen gespecificeerd worden door te splitsen met komma's)
- `<Enter><~><.>` : sluit een vastgelopen sessie

## Links
- More information: https://man.openbsd.org/ssh
