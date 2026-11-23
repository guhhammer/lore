# dumpntlminfo.py

Perform NTLM authentication against a remote host without credentials and dump information leaked in the NTLMSSP message. Part of the Impacket suite.

## Usage
- `DumpNTLMInfo.py target` : dump NTLM info from target (SMB, default port 445)
- `DumpNTLMInfo.py -target-ip target_ip target` : dump NTLM info using a specific IP
- `DumpNTLMInfo.py -port port target` : specify a custom port
- `DumpNTLMInfo.py -protocol RPC -port 135 target` : dump NTLM info using RPC protocol (default port 135)
- `DumpNTLMInfo.py -debug target` : turn on debug output

## Categories
network

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/dumpntlminfo.py
