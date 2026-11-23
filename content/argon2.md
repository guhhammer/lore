# argon2

Calculate Argon2 cryptographic hashes.

## Usage
- `echo "password" | argon2 "salt_text"` : calculate a hash with a password and a salt with the default parameters
- `echo "password" | argon2 "salt_text" -d|i|id` : calculate a hash with the specified algorithm
- `echo "password" | argon2 "salt_text" -e` : display the output encoded hash without additional information
- `echo "password" | argon2 "salt_text" -t 5 -m 20 -p 7` : calculate a hash with given iteration times, memory usage, and parallelism parameters

## Categories
users, security

## Links
- More information: https://github.com/P-H-C/phc-winner-argon2#command-line-utility
- tldr page: https://tldr.inbrowser.app/pages/common/argon2
