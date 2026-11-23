# echo

Toont gegeven argumenten.

## Usage
- `echo "Hallo Wereld"` : toon een tekstbericht. Let op: aanhalingstekens zijn optioneel
- `echo "Mijn pad is $PATH"` : toon een bericht met omgevingsvariabelen
- `echo -n "Hallo Wereld"` : toon een bericht zonder de afsluitende nieuwe regel
- `echo "Hallo Wereld" >>bestand.txt` : voeg een bericht aan een bestand toe
- `echo -e "kolom 1\tkolom 2"` : schakel interpretatie van backslash-escapes (speciale tekens) in
- `echo $?` : toon de afsluitstatus van de laatst uitgevoerde opdracht (Let op: in Windows Command Prompt en PowerShell zijn de equivalente opdrachten respectievelijk `echo %errorlevel%` en `$lastexitcode`)
- `echo "Hallo Wereld" | programma` : geef tekst door aan een ander programma via `stdin`

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
