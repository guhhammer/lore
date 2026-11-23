# gethelpcmd outlookcalendarchecktask

Scan for calendar configuration issues in legacy Microsoft Outlook applications. Note: This command is deprecated and will not work in the new Outlook application. Part of `GetHelpCmd.exe`, formerly `SaRAcmd.exe` (Microsoft Support and Recovery Assistant).

## Usage
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula` : analyze the calendar for the current, active profile, and accept this command's End-User License Agreement (EULA)
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula -P profile` : analyze calendar for a specific Outlook profile
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula -P profile -LogFolder path\to\directory` : analyze the calendar for a specific profile and save the logs to a specific directory
- `GetHelpCmd.exe -S OutlookCalendarCheckTask -AcceptEula -P profile -HideProgress` : analyze the calendar for a specific profile without showing the progress

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/gethelpcmd-outlookcalendarchecktask
