# syft

Generate a Software Bill of Materials (SBOM) from container images and filesystems.

## Usage
- `syft image:tag` : generate an SBOM from a container image
- `syft path/to/directory` : generate an SBOM from a local directory
- `syft path/to/archive.tar` : generate an SBOM from a container archive file
- `syft image:tag -o spdx-json` : generate an SBOM in SPDX JSON format
- `syft image:tag -o cyclonedx-json` : generate an SBOM in CycloneDX JSON format
- `syft image:tag -o format=path/to/output_file` : write the SBOM to a file in a specific format
- `syft -h` : display help
- `syft --version` : display version

## Categories
containers, media, files

## Links
- More information: https://oss.anchore.com/docs/reference/syft/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/syft
