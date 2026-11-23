# rsactftool.py

RSA attack tool for CTF challenges - recover private keys from weak public keys and/or decrypt data.

## Usage
- `RsaCtfTool.py --publickey path/to/key.pub --private` : recover a private key from a public key file
- `RsaCtfTool.py --publickey path/to/key.pub --decryptfile path/to/ciphered_file` : decrypt a file using a public key
- `RsaCtfTool.py --publickey path/to/key.pub --decrypt "ciphertext"` : decrypt a specific ciphertext string
- `RsaCtfTool.py --dumpkey --key path/to/key.pub` : dump RSA key components (e.g., modulus, exponent) from a key file
- `RsaCtfTool.py --publickey path/to/key.pub --private --attack fermat` : run a specific attack (e.g., Fermat factorization) to recover the private key
- `RsaCtfTool.py --createpub -n modulus -e exponent` : generate a public key from modulus (n) and exponent (e)
- `RsaCtfTool.py --publickey path/to/key.pub --private --attack all` : attempt all available attacks to recover the private key

## Categories
files, security

## Links
- More information: https://github.com/RsaCtfTool/RsaCtfTool#usage
- tldr page: https://tldr.inbrowser.app/pages/common/rsactftool.py
