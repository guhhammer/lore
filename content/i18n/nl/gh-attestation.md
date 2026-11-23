# gh attestation

Download en verifieer artefactverklaringen om hun integriteit en herkomst te controleren.

## Usage
- `gh at download pad/naar/artefact.bin -R eigenaar/repository` : download verklaringen voor een lokaal bestand dat aan een specifieke repository is gekoppeld
- `gh at download oci://image_uri -o organisatienaam` : download verklaringen voor een OCI-containerimage die aan een organisatie is gekoppeld
- `gh at verify pad/naar/artefact.bin -R eigenaar/repository` : verifieer een lokaal artefact online aan de hand van verklaringen van een specifieke repository
- `gh at verify pad/naar/artefact.bin -o organisatienaam --signer-workflow eigenaar/repository/pad/naar/workflow.yml` : verifieer een artefact en vereis dat het is ondertekend door een specifieke herbruikbare workflow voor verbeterde beveiliging
- `gh at verify pad/naar/artefact.bin -o organisatienaam --format json` : verifieer een artefact en toon de gedetailleerde verificatieresultaten als JSON voor gebruik in beleids-engines
- `gh at verify pad/naar/artefact.bin -b pad/naar/bundle.jsonl --custom-trusted-root pad/naar/vertrouwde_root.jsonl` : voer een volledig offline verificatie uit met een gedownload bundle en een aangepast vertrouwd root-bestand
- `gh at trusted-root >pad/naar/vertrouwde_root.jsonl` : sla de vertrouwde root van ondertekeningscertificaten op in een bestand voor offline verificatie

## Links
- More information: https://cli.github.com/manual/gh_attestation
