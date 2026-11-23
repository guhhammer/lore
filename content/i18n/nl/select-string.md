# select-string

Vindt tekst in strings en bestanden in PowerShell. Je kan `Select-String` gebruiken zoals `grep` in UNIX of `findstr.exe` in Windows. Opmerking: dit commando kan alleen gebruikt worden via PowerShell.

## Usage
- `Select-String -Path "pad\naar\bestand" -Pattern 'zoek_patroon'` : zoek naar een patroon binnen een bestand
- `Select-String -SimpleMatch "exacte_string" pad\naar\bestand` : zoek naar een exacte string (schakelt `regex` uit)
- `Select-String -Path "*.ext" -Pattern 'zoek_patroon'` : zoek naar een patroon in alle `.ext` bestanden in de huidige map
- `Select-String --Context 2,3 "zoek_patroon" pad\naar\bestand` : toon het opgegeven aantal regels voor en na de regel die overeenkomt met het patroon
- `Get-Content pad\naar\bestand | Select-String --NotMatch "zoek_patroon"` : zoek in `stdin` naar regels die niet overeenkomen met een patroon
