# matchpathcon

Zoek de persistente SELinux-beveiligingscontextinstelling van een pad op.

## Usage
- `matchpathcon /pad/naar/bestand` : zoek de persistente beveiligingscontextinstelling van een absoluut pad op
- `matchpathcon -m file|dir|pipe|chr_file|blk_file|lnk_file|sock_file /pad/naar/bestand` : beperk het opzoeken tot instellingen voor een specifiek bestandstype
- `matchpathcon -V /pad/naar/bestand` : verifieer dat de persistente en huidige beveiligingscontext van een pad overeenkomen

## Links
- More information: https://manned.org/matchpathcon.8
