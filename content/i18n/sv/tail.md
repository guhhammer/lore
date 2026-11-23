# tail

Via den sista delen av en fil.

## Usage
- `tail sökväg/till/fil` : visa de sista 10 raderna av en fil
- `tail sökväg/till/fil1 sökväg/till/fil2 ...` : visa de 10 sista raderna av flera olika filer
- `tail -5 sökväg/till/fil` : visa de sista 5 raderna av en fil
- `tail -n +antal sökväg/till/fil` : visa de sista `antal` raderna av en fil
- `tail -c antal sökväg/till/fil` : visa de sista `antal` bytes av en fil
- `tail -f sökväg/till/fil` : visa de sista raderna av en fil och fortsätt läsa till `<Ctrl c>`
- `tail -F sökväg/till/fil` : fortsätt läsa filen till `<Ctrl c>`, även om filen blir oåtkomlig
- `tail -n antal -s sekunder -f sökväg/till/fil` : visa de sista `antal` raderna i en fil och updatera varje `sekunder` sekunder

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
