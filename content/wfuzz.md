# wfuzz

A web application bruteforcer.

## Usage
- `wfuzz -w path/to/file -p 127.0.0.1:8080:HTTP http://example.com/FUZZ` : brute force directories and files using the specified wordlist and also proxy the traffic
- `wfuzz -w path/to/file -f filename http://example.com/FUZZ` : save the results to a file
- `wfuzz -c -w path/to/file --sc 200,301,302 http://example.com/FUZZ` : show colorized output while only showing the declared response codes in the output
- `wfuzz -w path/to/file -H "Host: FUZZ.example.com" --hc 301 --hw 222 -t 100 example.com` : use a custom Header to fuzz subdomains while hiding specific response codes and word counts. Increase the threads to 100 and include the target ip/domain
- `wfuzz -c --hc 401 -s delay_between_requests_in_seconds -z file,path/to/usernames -z file,path/to/passwords --basic 'FUZZ:FUZ2Z' https://example.com` : brute force Basic Authentication using a list of usernames and passwords from files for each FUZz keyword, hiding response codes of unsuccessful attempts
- `wfuzz -z list,word1-word2-... https://api.example.com -d "id=FUZZ&showwallet=true"` : provide wordlist directly from the command-line and use POST request for fuzzing
- `wfuzz -z file,path/to/file,none-base64-md5 https://example.com/FUZZ` : provide wordlists from a file applying Base64 and MD5 encoding on them (`wfuzz -e encoders` lists all available encoders)
- `wfuzz -e encoders|payloads|iterators|printers|scripts` : list available encoders/payloads/iterators/printers/scripts

## Categories
files, network

## Links
- More information: https://wfuzz.readthedocs.io/en/latest/user/basicusage.html
- tldr page: https://tldr.inbrowser.app/pages/common/wfuzz
