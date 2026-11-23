# dumpntlminfo.py

Voer NTLM-authenticatie uit tegen een externe host zonder inloggegevens en dump informatie die is gelekt in het NTLMSSP-bericht. Onderdeel van de Impacket-suite.

## Usage
- `DumpNTLMInfo.py doel` : dump NTLM-informatie van het doel (SMB, standaard poort 445)
- `DumpNTLMInfo.py -target-ip doel_ip doel` : dump NTLM-informatie met behulp van een specifiek IP
- `DumpNTLMInfo.py -port poort doel` : specificeer een aangepaste poort
- `DumpNTLMInfo.py -protocol RPC -port 135 doel` : dump NTLM-informatie met behulp van het RPC-protocol (standaard poort 135)
- `DumpNTLMInfo.py -debug doel` : schakel debug-uitvoer in

## Links
- More information: https://github.com/fortra/impacket
