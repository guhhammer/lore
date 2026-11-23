# integritysetup

Manage dm-integrity (block level integrity) volumes.

## Usage
- `integritysetup <action> [<options>] <action args>` : the general form, as the manual gives it
- `integritysetup --allow-discards` : allow the use of discard (TRIM) requests for the device
- `integritysetup --batch-mode` : do not ask for confirmation
- `integritysetup --bitmap-flush-time MS` : bitmap flush time in milliseconds
- `integritysetup --buffer-sectors SECTORS` : the number of sectors in one buffer

## Links
- Documentation: https://gitlab.com/cryptsetup/cryptsetup/wikis/DMIntegrity
