# dnf builddep

Installa le dipendenze per compilare un dato pacchetto. Non predefinito in `dnf` ma supportato tramite `dnf-plugins-core`.

## Usage
- `dnf builddep path/to/specification.spec` : installa le dipendenze per un dato pacchetto
- `dnf builddep --skip-unavailable path/to/specification.spec` : installa le dipendenze per un dato pacchetto ma ignora quelle non disponibili
- `dnf builddep -D 'expression'` : definisce il macro RPM con una data espressione
- `dnf builddep --spec argument` : definisce un argomento per un percorso file `.spec`
- `dnf builddep --srpm argument` : definisce un argomento per un percorso file `.rpm`
- `dnf builddep --help-cmd` : mostra l'aiuto

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/builddep.html
