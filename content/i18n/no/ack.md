# ack

Et søkeverktøy som grep, optimalisert for utviklere.

## Usage
- `ack "søkemønster"` : søk etter filer som inneholder en streng eller regulært uttrykk i gjeldende katalog rekursivt
- `ack -i "søkemønster"` : søk etter et mønster som ikke skiller mellom store og små bokstaver
- `ack -o "søkemønster"` : søk etter linjer som samsvarer med et mønster, skriv ut bare den samsvarende teksten og ikke resten av linjen
- `ack -t ruby "søkemønster"` : begrens søket til filer av en bestemt type
- `ack -t noruby "søkemønster"` : ikke søk i filer av en bestemt type
- `ack -c -h "søkemønster"` : tell totalt antall treff funnet
- `ack -c -l "søkemønster"` : skriv ut filnavnene og antall treff kun for hver fil
- `ack --help-types` : list opp alle verdiene som kan brukes med `--type`

## Links
- More information: https://beyondgrep.com/documentation/
