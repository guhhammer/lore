# alias

Maak een alias aan - woorden die vervangen worden door een commandostring. Aliassen verdwijnen aan het einde van de huidige shellsessie, tenzij ze gedefinieerd zijn in het configuratiebestand van de shell, bijvoorbeeld `~/.bashrc` voor Bash of `~/.zshrc` voor Zsh.

## Usage
- `alias` : toon alle aliassen
- `alias woord="commando"` : maak een generieke alias aan
- `alias woord` : laat het gekoppelde commando zien van een gegeven alias
- `unalias woord` : verwijder een alias
- `alias rm="rm --interactive"` : maak van `rm` een interactief commando
- `alias la="ls --all"` : maak een alias `la` aan als korte schrijfwijze van `ls --all`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
