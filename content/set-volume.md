# set-volume

Set or change the filesystem label of an existing volume. Note: This command can only be used through PowerShell.

## Usage
- `Set-Volume -DriveLetter "D" -NewFileSystemLabel "DataVolume"` : change the filesystem label of a volume identified by drive letter
- `Set-Volume -FileSystemLabel "OldLabel" -NewFileSystemLabel "NewLabel"` : change the filesystem label of a volume identified by the system label
- `Set-Volume -InputObject $(Get-Volume -DriveLetter "E") -NewFileSystemLabel "Backup"` : modify the properties of a volume using a volume object
- `Set-Volume -DriveLetter "D" -DedupMode Backup` : specify the Data Deduplication mode for the volume

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, windows

## Links
- More information: https://learn.microsoft.com/powershell/module/storage/set-volume
- tldr page: https://tldr.inbrowser.app/pages/windows/set-volume
