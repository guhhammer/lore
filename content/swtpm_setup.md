# swtpm_setup

Swtpm tool to simulate the manufacturing of a TPM 1.2 or 2.0.

## Usage
- `swtpm_setup [OPTIONS]` : the general form, as the manual gives it
- `swtpm_setup --runas <userid>` : use this userid to run swtpm_setup as
- `swtpm_setup --tpm2` : do setup on a TPM 2; by default a TPM 1.2 is setup
- `swtpm_setup --createek` : create an endorsement key (EK)
- `swtpm_setup --allow-signing` : create an EK that can sign
