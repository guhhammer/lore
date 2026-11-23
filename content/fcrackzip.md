# fcrackzip

Fast password cracker for encrypted ZIP archives.

## Usage
- `fcrackzip -D -p path/to/wordlist.txt path/to/file.zip` : perform a dictionary attack using a wordlist
- `fcrackzip -v -u -D -p path/to/wordlist.txt path/to/file.zip` : perform a dictionary attack with verbose output and verify password guesses
- `fcrackzip -b -c a -l min-max path/to/file.zip` : brute-force a password using lowercase letters within a length range
- `fcrackzip -b -c a1 path/to/file.zip` : brute-force a password using lowercase letters and digits

## Categories
files, archives, users

## Links
- More information: https://github.com/hyc/fcrackzip
- tldr page: https://tldr.inbrowser.app/pages/common/fcrackzip
