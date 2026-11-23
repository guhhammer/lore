# gethelpcmd officeactivationscenario

Herstel automatisch activeringsgerelateerde problemen binnen Microsoft Office / Microsoft 365 Apps for Enterprise. Opmerking: deze tool is verouderd en werkt niet in de nieuwe OneNote- en Outlook-applicaties. Opmerking: dit commando kan je huidige volume aan gelicentieerde Office-productversies overschrijven, deactiveren en/of verwijderen, ga dus voorzichtig te werk. Onderdeel van `GetHelpCmd.exe`, voorheen `SaRAcmd.exe` (Microsoft Support and Recovery Assistant).

## Usage
- `GetHelpCmd.exe -S OfficeActivationScenario -AcceptEula -CloseOffice` : herstel automatisch activeringsgerelateerde fouten, sluit Office af, en accepteer de licentieovereenkomst voor eindgebruikers (EULA) van dit commando. Vereist Administrator-rechten
- `GetHelpCmd.exe -S OfficeActivationScenario -AcceptEula -CloseOffice -RemoveSCA` : ontkoppel het momenteel geïnstalleerde Office-licentieschema van Shared Computer Activation (SCA), en activeer het product afzonderlijk. Vereist Administrator-rechten
