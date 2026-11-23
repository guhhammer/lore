# gethelpcmd officeactivationscenario

Automatically repair activation-related issues within Microsoft Office / Microsoft 365 Apps for Enterprise. Note: This command is deprecated and will not work in the new OneNote and Outlook applications. Note: This command may override, deactivate, and/or remove your current volume of licensed Office product versions, so please proceed cautiously. Part of `GetHelpCmd.exe`, formerly `SaRAcmd.exe` (Microsoft Support and Recovery Assistant).

## Usage
- `GetHelpCmd.exe -S OfficeActivationScenario -AcceptEula -CloseOffice` : automatically fix activation-related errors, close Office, and accept this command's End-User License Agreement (EULA). Requires Administrator privileges to do so
- `GetHelpCmd.exe -S OfficeActivationScenario -AcceptEula -CloseOffice -RemoveSCA` : detach the currently-installed Office licensing scheme from Shared Computer Activation (SCA), and activate the product separately. Requires Administrator privileges to do so

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## See also
- ospp.vbs

## Categories
documents, disks, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/gethelpcmd-officeactivationscenario
