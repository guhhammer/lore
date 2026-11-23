# mktemp

Maak een tijdelijk bestand of een tijdelijke map aan.

## Usage
- `mktemp` : maak een leeg tijdelijk bestand en toon het absolute pad
- `mktemp -p /pad/naar/tijdelijke_map` : gebruik een aangepaste map als `$TMPDIR` niet is ingesteld (de standaard is platformafhankelijk, maar meestal `/tmp`)
- `mktemp /tmp/voorbeeld.XXXXXXXX` : gebruik een aangepast pad-sjabloon (`X`en worden vervangen door willekeurige alfanumerieke tekens)
- `mktemp -t voorbeeld.XXXXXXXX` : gebruik een aangepast bestandsnaam-sjabloon
- `mktemp -d` : maak een lege tijdelijke map aan en toon het absolute pad

## Links
- More information: https://man.openbsd.org/mktemp.1
