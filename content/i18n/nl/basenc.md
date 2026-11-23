# basenc

Encodeer of decodeer een bestand of `stdin` door gebruik te maken van een specifieke encoding naar `stdout`.

## Usage
- `basenc --base64 pad/naar/bestand` : encodeer een bestand met Base64 encoding
- `basenc -d --base64 pad/naar/bestand` : decodeer een bestand met Base64 encoding
- `commando | basenc --base32 -w 42` : encodeer `stdin` met Base32 encoding met 42 kolommen
- `commando | basenc --base32` : encodeer `stdin` met Base32 encoding
