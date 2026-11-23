# slmgr.vbs

Install, activate, and manage Windows licenses. Use `cscript` to run this program as a CLI, or `wscript` as a GUI. Note: This command may override, deactivate, and/or remove your current Windows license, so please proceed cautiously.

## Usage
- `cscript slmgr.vbs /dli` : display the current Windows license information
- `cscript slmgr.vbs /dti` : display the installation iD for the current device. Useful for offline license activation
- `cscript slmgr.vbs /xpr` : display the current license's e[xp]iration date and time
- `cscript slmgr.vbs /ipk product_key` : install a new Windows license product key. Requires Administrator privileges and will override the existing license
- `cscript slmgr.vbs /ato` : activate the Windows product license online. Requires Administrator privileges to do so
- `cscript slmgr.vbs /atp confirmation_id` : activate the Windows product license offline. Requires Administrator privileges and a Confirmation ID provided by Microsoft Product Activation Center
- `cscript slmgr.vbs /cpky` : clear the current license's product key from the Windows Registry. This will not deactivate or uninstall the current license, but prevents the key from being stolen by malicious programs in the future
- `cscript slmgr.vbs /upk` : uninstall the current license (by its product key)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/slmgr.vbs
