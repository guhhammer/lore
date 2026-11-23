# get-alias

Toon en verkrijg commandoaliassen in de huidige PowerShell-sessie. Dit commando kan alleen worden uitgevoerd onder PowerShell.

## Usage
- `Get-Alias` : toon alle aliassen in de huidige sessie
- `Get-Alias commando_alias` : haal de gealiaste commandonaam op
- `Get-Alias -Definition commando` : toon alle aliassen toegewezen aan een specifiek commando
- `Get-Alias abc* -Exclude *def` : toon aliassen die beginnen met `abc`, maar sluit die uit die eindigen op `def`
