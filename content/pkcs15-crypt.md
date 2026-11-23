# pkcs15-crypt

Perform crypto operations using PKCS#15 smart cards.

## Usage
- `pkcs15-crypt [OPTIONS]` : the general form, as the manual gives it
- `pkcs15-crypt --aid aid` : specify the AID of the on-card PKCS#15 application to bind to
- `pkcs15-crypt --decipher` : decrypt the contents of the file specified by the --input option
- `pkcs15-crypt --input file` : specifies the input file to use
- `pkcs15-crypt --key id` : selects the ID of the key to use

## Categories
files
