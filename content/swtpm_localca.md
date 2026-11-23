# swtpm_localca

Local CA to create EK and platform certs for swtpm.

## Usage
- `swtpm_localca [OPTIONS]` : the general form, as the manual gives it
- `swtpm_localca --type type` : this parameter indicates the type of certificate to create
- `swtpm_localca --tpm2` : create TPM 2 compliant certificates
- `swtpm_localca --allow-signing` : create an EK that can also be used for signing
