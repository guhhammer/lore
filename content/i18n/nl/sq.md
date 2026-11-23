# sq

Een moderne OpenPGP command-line tool.

## Usage
- `sq encrypt --with-password --without-signature pad/naar/bestand --output pad/naar/bestand.pgp` : versleutel een bestand met een wachtwoord (symmetrische versleuteling)
- `sq decrypt pad/naar/bestand.pgp --output pad/naar/bestand` : ontsleutel een met wachtwoord beveiligd bestand
- `sq inspect pad/naar/bestand.pgp` : inspecteer een OpenPGP-bestand om de metadata en structuur te bekijken
- `sq verify --signer-file pad/naar/ondertekenaar.asc --signature-file pad/naar/bestand.sig pad/naar/bestand` : verifieer een bestand met een losstaande handtekening en een certificaatbestand
- `sq verify --signer-file pad/naar/ondertekenaar.asc --cleartext pad/naar/bestand` : verifieer een bestand met een ingesloten (leesbare) handtekening en een certificaatbestand
- `sq key generate --own-key --name naam --email naam@example.com` : genereer je eigen sleutel en sla deze op in de lokale sleutelopslag
- `sq key|cert list` : toon alle geheime sleutels of certificaten die beheerd worden door de lokale sleutelopslag
- `sq config inspect paths` : toon de huidige configuratie en opslagpaden

## Links
- More information: https://sequoia-pgp.gitlab.io/sequoia-sq/man/sq.1.html
