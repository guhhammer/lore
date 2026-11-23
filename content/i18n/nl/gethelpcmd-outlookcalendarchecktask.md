# gethelpcmd outlookcalendarchecktask

Scan op agendaconfiguratieproblemen in verouderde Microsoft Outlook-applicaties. Opmerking: deze tool is verouderd en werkt niet in de nieuwe Outlook-applicatie. Onderdeel van `GetHelpCmd.exe`, voorheen `SaRAcmd.exe` (Microsoft Support and Recovery Assistant).

## Usage
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula` : analyseer de agenda voor het huidige, actieve profiel, en accepteer de licentieovereenkomst voor eindgebruikers (EULA) van dit commando
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula -P profiel` : analyseer de agenda voor een specifiek Outlook-profiel
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula -P profiel -LogFolder pad\naar\map` : analyseer de agenda voor een specifiek profiel en sla de logs op in een specifieke map
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula -P profiel -HideProgress` : analyseer de agenda voor een specifiek profiel zonder de voortgang te tonen
