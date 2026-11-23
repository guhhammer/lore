# psversiontable

A read-only variable (as `$PSVersionTable`) to get the current PowerShell version. This command can only be run under PowerShell.

## Usage
- `$PSVersionTable` : print a summary of the currently installed PowerShell version and edition
- `$PSVersionTable.PSVersion` : get the detailed (major, minor, build, and revision) version number of PowerShell
- `$PSVersionTable.PSCompatibleVersions` : list all supported PowerShell script versions that this PowerShell version supports
- `$PSVersionTable.GitCommitId` : get the latest Git commit ID where the currently-installed PowerShell version is based on (works on PowerShell 6.0 and later)
- `$PSVersionTable.PSEdition` : check whether the user is running PowerShell Core (6.0 or later) or the original "Windows PowerShell" (version 5.1 or below)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/psversiontable
