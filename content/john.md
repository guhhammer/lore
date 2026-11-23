# john

Password cracker.

## Usage
- `john path/to/hashes.txt` : crack password hashes
- `john --show path/to/hashes.txt` : show passwords cracked
- `john --show --users=user_ids path/to/hashes1.txt path/to/hashes2.txt ...` : display users' cracked passwords by user identifier from multiple files
- `john --wordlist=path/to/wordlist.txt path/to/hashes.txt` : crack password hashes, using a custom wordlist
- `john --list=formats` : list available hash formats
- `john --format=md5crypt path/to/hashes.txt` : crack password hashes, using a specific hash format
- `john --rules path/to/hashes.txt` : crack password hashes, enabling word mangling rules
- `john --restore=path/to/mycrack.rec` : restore an interrupted cracking session from a state file, e.g. `mycrack.rec`

## Categories
files, users

## Links
- More information: https://www.openwall.com/john/
- tldr page: https://tldr.inbrowser.app/pages/common/john
