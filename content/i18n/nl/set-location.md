# set-location

Geef de huidige werkmap weer of ga naar een andere map. Opmerking: dit commando kan alleen gebruikt worden via PowerShell.

## Usage
- `Set-Location pad\naar\map` : ga naar de opgegeven map
- `Set-Location C:pad\naar\map` : ga naar een map in een andere drive
- `Set-Location pad\naar\map -PassThru` : ga en toon de locatie van de opgegeven map
- `Set-Location ..` : ga naar de bovenliggende map van de huidige map
- `Set-Location ~` : ga naar de thuismap van de huidige gebruiker
- `Set-Location -|+` : ga terug/vooruit naar de eerder gekozen map
- `Set-Location \` : ga naar de hoofdmap van de huidige drive
