# searchsploit

Search Exploit Database for exploits, shellcodes and/or papers. If known version numbers are used as search terms, exploits for both the exact version and others whose version range covers the one specified are shown.

## Usage
- `searchsploit search_terms` : search for an exploit, shellcode, or paper
- `searchsploit sudo 1.8.27` : search for a known specific version, e.g. sudo version 1.8.27
- `searchsploit -w search_terms` : show the exploit-db link to the found resources
- `searchsploit -m exploit_number` : copy the resource to the current directory (requires the number of the exploit)
- `searchsploit -x exploit_number` : examine the resource, using the pager defined in the `$PAGER` environment variable
- `searchsploit -u` : update the local Exploit Database
- `searchsploit --cve 2021-44228` : search for the common vulnerabilities and exposures (CVE) value
- `searchsploit --nmap path/to/nmap-output.xml` : check results in `nmap`'s XML output with service version (`nmap -sV -oX nmap-output.xml`) for known exploits

## Categories
databases, files, text

## Links
- More information: https://www.exploit-db.com/searchsploit
- tldr page: https://tldr.inbrowser.app/pages/common/searchsploit
