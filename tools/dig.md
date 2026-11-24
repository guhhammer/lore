# dig

dig asks a DNS server a question and prints the whole answer. It is the tool to see what a name really resolves to and which server says so.

## Tasks
### Look up a name
- `dig example.org` : the A record (IPv4 address) with full details
- `dig +short example.org` : only the answer
- `dig AAAA example.org` : the IPv6 address
- `dig MX example.org` : mail servers
- `dig TXT example.org` : text records (SPF, verification codes)
- `dig NS example.org` : the name servers of the domain
- `dig CNAME www.example.org` : the alias target
- `dig ANY example.org` : ask for everything (many servers refuse it)
### Choose the server
- `dig @1.1.1.1 example.org` : ask one resolver directly
- `dig @ns1.example.org example.org` : ask the domain's own server, no cache
- `dig +trace example.org` : walk the lookup from the root servers down
- `dig +norecurse @1.1.1.1 example.org` : only what that server already has cached
### Reverse and detail
- `dig -x 8.8.8.8` : reverse lookup, the name of an address
- `dig +noall +answer example.org` : only the answer section
- `dig +noall +answer +ttl example.org` : answer with the time left in cache
- `dig +dnssec example.org` : ask for DNSSEC records
- `dig SOA example.org` : the zone's serial number and timers
- `dig +tcp example.org` : use TCP instead of UDP
### Many names
- `dig +short -f NAMES.txt` : look up every name in a file
- `dig +short example.org @8.8.8.8 @1.1.1.1` : compare answers from two resolvers
- `dig +nssearch example.org` : ask every name server for the serial, to see if they agree

## Notes
- A fresh DNS change can look wrong for as long as the old record's TTL: ask the authoritative server to see the truth.
- dig skips the local `/etc/hosts` file; `getent hosts NAME` does not.
- `ANY` queries are often refused or cut short now.
- Install it with the package `dnsutils` (Debian, Ubuntu), `bind-utils` (Fedora) or `bind` (Arch).

## See also
- host
- nslookup
- resolvectl
- whois

## Links
- Manual: https://bind9.readthedocs.io/en/latest/manpages.html#dig-dns-lookup-utility

## Categories
network
