# get-acl

Get the security descriptor for a resource, such as a file or registry key. Note: This command can only be used through PowerShell.

## Usage
- `Get-Acl path\to\directory` : display the ACL for a specific directory
- `Get-Acl -Path HKLM:\System\CurrentControlSet\Control | Format-List` : get an ACL for a registry key

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/get-acl
