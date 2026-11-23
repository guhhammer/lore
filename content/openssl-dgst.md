# openssl dgst

Generate digest values and perform signature operations.

## Usage
- `openssl dgst -sha256 -binary -out output_file input_file` : calculate the SHA256 digest for a file, saving the result to a specific file
- `openssl dgst -sign private_key_file -sha256 -sigopt rsa_padding_mode:pss -out output_file input_file` : sign a file using an RSA key, saving the result to a specific file
- `openssl dgst -verify public_key_file -signature signature_file -sigopt rsa_padding_mode:pss signature_message_file` : verify an RSA signature
- `openssl dgst -sign private_key_file -sha256 -out output_file input_file` : sign a file using and ECDSA key
- `openssl dgst -verify public_key_file -signature signature_file signature_message_file` : verify an ECDSA signature

## Categories
security, files

## Links
- More information: https://docs.openssl.org/master/man1/openssl-dgst/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-dgst
