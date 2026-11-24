# hashcat

Fast and advanced password recovery tool.

## Usage
- `hashcat -m hash_type_id -a 3 hash_value` : perform a brute-force attack (mode 3) with the default hashcat mask
- `hashcat -m hash_type_id -a 3 hash_value "?d?d?d?d"` : perform a brute-force attack (mode 3) with a known pattern of 4 digits
- `hashcat -m hash_type_id -a 3 --increment hash_value "?a?a?a?a?a?a?a?a"` : perform a brute-force attack (mode 3) using at most 8 of all printable ASCII characters
- `hashcat -m hash_type_id -a 0 hash_value path/to/wordlist.txt` : perform a dictionary attack (mode 0) using a wordlist
- `hashcat -m hash_type_id -a 0 -r path/to/file.rule hash_value path/to/wordlist.txt` : run a dictionary attack (mode 0) using the specified wordlist, applying rule-based transformations to mutate candidate passwords
- `hashcat -m hash_type_id -a 1 hash_value path/to/dictionary1.txt path/to/dictionary2.txt` : perform a combination attack (mode 1) using the concatenation of words from two different custom dictionaries
- `hashcat --show hash_value` : show result of an already cracked hash
- `hashcat --example-hashes` : show all example hashes

## Categories
files, users

## Links
- More information: https://hashcat.net/wiki/doku.php?id=hashcat
- tldr page: https://tldr.inbrowser.app/pages/common/hashcat
- Wikipedia: https://en.wikipedia.org/wiki/Hashcat
