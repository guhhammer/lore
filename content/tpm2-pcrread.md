# tpm2 pcrread

Read the PCR values from a TPM.

## Usage
- `tpm2 pcrread` : read all PCR values
- `tpm2 pcrread sha256:all` : read all PCR values of the sha256 bank
- `tpm2 pcrread -o path/to/file sha1:16,17,18+sha256:16,17,18` : read specific PCR values from multiple banks and write them to a file

## Categories
files

## Links
- More information: https://manned.org/tpm2_pcrread
- tldr page: https://tldr.inbrowser.app/pages/common/tpm2-pcrread
