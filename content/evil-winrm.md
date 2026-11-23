# evil-winrm

Windows Remote Management (WinRM) shell for pentesting. Once connected, we get a PowerShell prompt on the target host.

## Usage
- `evil-winrm -i ip_address -u user -p password` : connect to a host and start an interactive session
- `evil-winrm -i ip_address -u user -H nt_hash` : connect to a host using pass-the-hash authentication instead of a password
- `evil-winrm -i ip_address -u user -p password -s path/to/scripts -e path/to/executables` : connect to a host, specifying directories for PowerShell scripts and executables
- `evil-winrm -i ip_address -u user -p password -S -c path/to/pubkey -k path/to/privkey` : connect to a host, using SSL
- `upload path/to/local_file path/to/remote_file` : [Interactive] Upload a file to the host
- `menu` : [Interactive] List all loaded PowerShell functions
- `script.ps1` : [Interactive] Load a PowerShell script from the `--scripts` directory
- `Invoke-Binary binary.exe` : [Interactive] Invoke a binary on the host from the `--executables` directory

## Categories
network, files, users

## Links
- More information: https://github.com/Hackplayers/evil-winrm
- tldr page: https://tldr.inbrowser.app/pages/common/evil-winrm
