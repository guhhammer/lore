# gpgsplit

Split an OpenPGP message into packets.

## Usage
- `gpgsplit [OPTIONS] [FILES]` : the general form, as the manual gives it
- `gpgsplit -v` : verbose
- `gpgsplit -p` : prepend filenames with STRING
- `gpgsplit --uncompress` : uncompress a packet
- `gpgsplit --secret-to-public` : convert secret keys to public keys
