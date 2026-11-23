# pwsh

Shell en scriptingtaal specifiek ontworpen voor systeemadministratie. Dit commando refereert naar PowerShell versie 6 en hoger (ook wel bekend als PowerShell Core en cross-platform PowerShell). Om de originele Windows versie (5.1 en lager, ook wel bekend als de legacy Windows PowerShell) te gebruiken, gebruik `powershell` in plaats van `pwsh`.

## Usage
- `pwsh` : start een interactieve shellsessie
- `pwsh -NoProfile` : start een interactieve shellsessie zonder het laden van startup configuraties
- `pwsh -Command "echo 'powershell is uitgevoerd'"` : voer specifieke commando's uit
- `pwsh -File pad/naar/script.ps1` : voer een specifiek script uit
- `pwsh -Version versie` : start een sessie met een specifieke versie van PowerShell
- `pwsh -NoExit` : voorkom dat een shell afsluit na het uitvoeren van de opstart-commando's
- `pwsh -InputFormat Text|XML` : beschrijf het formaat van de data die gestuurd wordt naar PowerShell
- `pwsh -OutputFormat Text|XML` : bepaal hoe een uitvoer van PowerShell wordt geformatteerd
