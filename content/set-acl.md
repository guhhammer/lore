# set-acl

Change the security descriptor of a specified item, such as a file or a registry key. Note: This command can only be used through PowerShell.

## Usage
- `$OriginAcl = Get-Acl -Path path\to\file; Set-Acl -Path path\to\file -AclObject $OriginAcl` : copy a security descriptor from one file to another
- `Get-Acl -Path path\to\file | Set-Acl -Path path\to\file` : use the pipeline operator to pass a descriptor

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/set-acl
