# dnswalk

DNS debugger. "Walk" across zones and validate database consistency and best practices.

## Usage
- `dnswalk domain.` : debug a DNS pathway for a Fully Qualified Domain Name (FQDN)
- `dnswalk -r domain.` : process sub-domains recursively
- `dnswalk -m domain.` : only perform a `dnswalk` if the zone has been modified since the last run
- `dnswalk -d domain.` : print debugging and status information to `stderr` instead of `stdout`
- `dnswalk -i domain.` : suppress the check for invalid characters in the domain name
- `dnswalk -a domain.` : enable duplicate A record warnings
- `dnswalk -F domain.` : enable "Fascist checking" to compare the A record PTR name with the forward name and report mismatches
- `dnswalk -l domain.` : enable "lame delegation" to test whether the listed host is returning authoritative answers

## Categories
network, development, databases

## Links
- More information: https://manned.org/dnswalk
- tldr page: https://tldr.inbrowser.app/pages/common/dnswalk
