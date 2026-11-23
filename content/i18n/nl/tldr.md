# tldr

Toon simpele hulppagina's voor command-line programma's uit het tldr-pages project. Opmerking: De opties `--language` en `--list` zijn niet vereist door de clientspecificatie, maar de meeste clients implementeren ze wel.

## Usage
- `tldr commando` : toon de tldr-pagina voor een specifiek commando (hint: dit is hoe je hier bent gekomen!)
- `tldr commando subcommando` : toon de tldr-pagina voor een specifiek subcommando
- `tldr -L taalcode commando` : toon de tldr-pagina voor een commando in de opgegeven taal (indien beschikbaar, val anders terug op Engels)
- `tldr -p android|cisco-ios|common|dos|freebsd|linux|netbsd|openbsd|osx|sunos|windows commando` : toon de tldr-pagina voor een commando van een specifiek platform
- `tldr -u` : update de lokale cache van tldr-pagina's
- `tldr -l` : toon alle pagina's voor het huidige platform en `common`
- `tldr --short-options commando` : toon de tldr-pagina voor een commando en toon korte opties
- `tldr -l | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr` : blader door tldr-pagina's in een terminalvenster (`fzf` moet beschikbaar zijn)
