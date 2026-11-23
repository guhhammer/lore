# gh attestation

Download and verify artifact attestations to ensure their integrity and provenance.

## Usage
- `gh at download path/to/artifact.bin -R owner/repository` : download attestations for a local file associated with a specific repository
- `gh at download oci://image_uri -o organization_name` : download attestations for an OCI container image associated with an organization
- `gh at verify path/to/artifact.bin -R owner/repository` : verify a local artifact online against attestations from a specific repository
- `gh at verify path/to/artifact.bin -o organization_name --signer-workflow owner/repository/path/to/workflow.yml` : verify an artifact, requiring it was signed by a specific reusable workflow for enhanced security
- `gh at verify path/to/artifact.bin -o organization_name --format json` : verify an artifact and output the detailed verification results as JSON for use in policy engines
- `gh at verify path/to/artifact.bin -b path/to/bundle.jsonl --custom-trusted-root path/to/trusted_root.jsonl` : perform a fully offline verification using a downloaded bundle and a custom trusted root file
- `gh at trusted-root >path/to/trusted_root.jsonl` : save the trusted root of signing certificates to a file for offline verification

## Categories
files, network, packages

## Links
- More information: https://cli.github.com/manual/gh_attestation
- tldr page: https://tldr.inbrowser.app/pages/common/gh-attestation
