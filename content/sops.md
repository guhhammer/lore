# sops

SOPS (Secrets OPerationS): a simple and flexible tool for managing secrets.

## Usage
- `sops -e path/to/file.json >path/to/file.enc.json` : encrypt a file
- `sops -d path/to/file.enc.json` : decrypt a file to `stdout`
- `sops updatekeys path/to/file.enc.yaml` : update the declared keys in a `sops` file
- `sops -r path/to/file.enc.yaml` : rotate data keys for a `sops` file
- `sops -d --input-type json path/to/file.enc.json` : change the extension of the file once encrypted
- `sops -d --extract '["an_array"][1]' path/to/file.enc.json` : extract keys by naming them, and array elements by numbering them
- `diff <(sops -d path/to/secret1.enc.yaml) <(sops -d path/to/secret2.enc.yaml)` : show the difference between two `sops` files

## Categories
files, text

## Links
- More information: https://github.com/getsops/sops
- tldr page: https://tldr.inbrowser.app/pages/common/sops
