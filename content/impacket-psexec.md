# impacket-psexec

This command is an alias of `psexec.py`.

## Usage
- `psexec.py domain/username:password@target` : spawn an interactive shell on a remote target
- `psexec.py domain/username:password@target command` : execute a specific command on a remote target
- `psexec.py -c filename domain/username:password@target command` : copy the filename for later execution, arguments are passed in the command
- `psexec.py -path "path\to\remote_directory" domain/username:password@target command` : execute a command from a specific path on a remote target
- `psexec.py -hashes LM_Hash:NT_Hash domain/username@target` : authenticate using pass-the-hash authentication instead of a password
- `psexec.py -k -no-pass domain/username@target` : use Kerberos authentication for the target
- `psexec.py -dc-ip domain_controller_ip domain/username:password@target` : specify the IP address of the domain controller

## Categories
users, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-psexec
