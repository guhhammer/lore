# ospp.vbs

Install, activate, and manage volume licensed versions of Microsoft Office products. Use `cscript` to run this program in the CLI, or `wscript` in the GUI. Note: This command may override, deactivate, and/or remove your current volume of licensed Office product versions, so please proceed cautiously.

## Usage
- `cscript ospp.vbs /inpkey:product_key` : install a product key (Note: It replaces the existing key)
- `cscript ospp.vbs /unpkey:product_key_digits` : uninstall an installed product key with the last five digits of the product key
- `cscript ospp.vbs /sethst:ip|hostname` : set a KMS host name
- `cscript ospp.vbs /setprt:port` : set a KMS port
- `cscript ospp.vbs /act` : activate installed Office product keys
- `cscript ospp.vbs /dstatus` : display license information for installed product keys

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## See also
- gethelpcmd-officeactivationscenario

## Categories
documents, disks, network, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/ospp.vbs
