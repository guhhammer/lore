# get-help

Toon hulpinformatie en documentatie voor PowerShell-commando's (aliassen, cmdlets en functies). Dit commando kan alleen worden uitgevoerd via PowerShell.

## Usage
- `Get-Help commando` : toon algemene hulpinformatie voor een specifiek PowerShell-commando
- `Get-Help commando -Detailed` : toon meer gedetailleerde documentatie voor een specifiek PowerShell-commando
- `Get-Help commando -Full` : toon de volledige technische documentatie voor een specifiek PowerShell-commando
- `Get-Help commando -Parameter parameter` : print alleen de documentatie voor een specifieke parameter van het PowerShell-commando (gebruik `*` om alle parameters te tonen), indien beschikbaar
- `Get-Help commando -Examples` : print alleen de voorbeelden van de cmdlet, indien beschikbaar
- `Get-Help *` : toon alle beschikbare cmdlet-hulppagina's
- `Update-Help` : werk de huidige hulp- en documentatiekennisbank bij met `Update-Help`
- `Get-Help commando -Online` : bekijk een online versie van de PowerShell-commandodocumentatie in de standaard webbrowser
