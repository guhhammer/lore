# sn

Mono StrongName utility for signing and verifying IL assemblies.

## Usage
- `sn -k path/to/key.snk` : generate a new StrongNaming key
- `sn -R path/to/assembly.dll path/to/key_pair.snk` : re-sign an assembly with the specified private key
- `sn -T path/to/assembly.exe` : show the public key of the private key that was used to sign an assembly
- `sn -e path/to/assembly.dll path/to/output.pub` : extract the public key to a file

## Categories
files

## Links
- More information: https://manned.org/sn
- tldr page: https://tldr.inbrowser.app/pages/common/sn
