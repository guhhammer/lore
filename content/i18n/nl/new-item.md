# new-item

Maak een nieuw bestand, map, symbolische link of een registerinvoer. Opmerking: dit commando kan alleen worden uitgevoerd onder PowerShell.

## Usage
- `New-Item pad\naar\bestand` : maak een nieuw leeg bestand (gelijk aan `touch`)
- `New-Item -ItemType Directory pad\naar\map` : maak een nieuwe map
- `New-Item pad\naar\bestand -Value inhoud` : schrijf een nieuw tekstbestand met opgegeven inhoud
- `New-Item pad\naar\bestand1 , pad\naar\bestand2 , ... -Value inhoud` : schrijf hetzelfde tekstbestand op meerdere locaties
- `New-Item -ItemType SymbolicLink|HardLink|Junction -Path pad\naar\linkbestand -Target pad\naar\bronbestand_of_map` : maak een symbolische link\harde link\junction naar een bestand of map
- `New-Item pad\naar\registersleutel` : maak een nieuwe lege registerinvoer (in REG_SZ, gebruik `New-ItemProperty` of `Set-ItemProperty` om het waardetype te verfijnen)
- `New-Item pad\naar\registersleutel -Value waarde` : maak een nieuwe lege registerinvoer met gespecificeerde waarde
