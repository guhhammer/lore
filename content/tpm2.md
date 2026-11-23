# tpm2

Run various tpm2-tools via a single unified executable. Some subcommands such as `pcrread`, `pcrreset`, etc. have their own usage documentation.

## Usage
- `tpm2 pcrreset 16` : reset the PCR 16 banks
- `tpm2 pcrextend 16:sha1=f1d2d2f924e986ac86fdf7b36c94bcdf32beec15` : extend the PCR 16 sha1 bank with `f1d2d2f924e986ac86fdf7b36c94bcdf32beec15`
- `tpm2 pcrread sha1:16` : read the PCR 16 sha1 bank
- `tpm2 -V subcommand` : run a subcommand in verbose mode
- `tpm2 -Q subcommand` : run a subcommand without writing to `stdout`
- `tpm2 -h` : display help

## Categories
documents, monitoring

## Links
- More information: https://manned.org/tpm2
- tldr page: https://tldr.inbrowser.app/pages/common/tpm2
