# pkcs15-tool

Utility for manipulating PKCS #15 data structures on smart cards and similar security tokens.

## Usage
- `pkcs15-tool [OPTIONS]` : the general form, as the manual gives it
- `pkcs15-tool --version` : print the OpenSC package release version
- `pkcs15-tool --auth-id id` : specifies the auth id of the PIN to use for the operation
- `pkcs15-tool --change-pin` : changes a PIN or PUK stored on the token
- `pkcs15-tool --dump` : list all card objects
