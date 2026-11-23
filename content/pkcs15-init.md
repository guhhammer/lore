# pkcs15-init

Smart card personalization utility.

## Usage
- `pkcs15-init [OPTIONS]` : the general form, as the manual gives it
- `pkcs15-init --card-profile name` : tells pkcs15-init to load the specified card profile option
- `pkcs15-init --serial SERIAL` : specify the serial number of the card
- `pkcs15-init --erase-application AID` : this will erase the application with the application identifier AID
- `pkcs15-init --generate-key keyspec` : tells the card to generate new key and store it on the card
