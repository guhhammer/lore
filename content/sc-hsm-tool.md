# sc-hsm-tool

Smart card utility for SmartCard-HSM.

## Usage
- `sc-hsm-tool --initialize` : initialize token, removing all existing keys, certificates and files
- `sc-hsm-tool --dkek-shares number-of-shares` : define the number of DKEK shares to use for recreating the DKEK
- `sc-hsm-tool --pin-retry value` : define number of PIN retries for user PIN during initialization
- `sc-hsm-tool --password value` : define password for DKEK share encryption

## Categories
users
