# pip index

Ispeziona le informazioni disponibili dagli indici dei pacchetti.

## Usage
- `pip index versions pacchetto` : elenca tutte le versioni disponibili di un pacchetto
- `pip index versions pacchetto --index-url https://test.pypi.org/simple/` : elenca le versioni da un indice specifico
- `pip index versions pacchetto --pre` : includi le versioni pre-rilascio
- `pip index versions pacchetto --extra-index-url https://example.com/simple/` : includi un indice aggiuntivo
- `pip index versions pacchetto --platform linux_x86_64` : elenca le versioni per una specifica piattaforma

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_index/
