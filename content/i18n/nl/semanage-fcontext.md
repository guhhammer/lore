# semanage fcontext

Beheer persistente SELinux-beveiligingscontextregels op bestanden/mappen.

## Usage
- `sudo semanage fcontext -l` : toon alle bestandslabelregels
- `sudo semanage fcontext -lCn` : toon alle door de gebruiker gedefinieerde bestandslabelregels zonder koppen
- `sudo semanage fcontext -a -t samba_share_t '/mnt/share(/.*)?'` : voeg een door de gebruiker gedefinieerde regel toe die elk pad labelt dat overeenkomt met een PCRE-`regex`
- `sudo semanage fcontext -a -e /pad/naar/ref /pad/naar/doel` : voeg een door de gebruiker gedefinieerde regel toe die labelgelijkwaardigheid creëert tussen twee subpaden
- `sudo semanage fcontext -d '/mnt/share(/.*)?'` : verwijder een door de gebruiker gedefinieerde regel met behulp van zijn PCRE-`regex`
- `restorecon -Rv pad/naar/map` : herlabel een map recursief door de nieuwe regels toe te passen

## Links
- More information: https://manned.org/semanage-fcontext
