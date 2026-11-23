# ldapdomaindump

Dump users, computers, groups, OS, and membership information via LDAP to HTML, JSON, and greppable output.

## Usage
- `ldapdomaindump -u domain\username -p password|ntlm_hash hostname|ip` : dump all information using the given LDAP account
- `ldapdomaindump -r -u domain\username -ppassword hostname|ip` : dump all information, resolving computer hostnames
- `ldapdomaindump -r -n domain_controller_ip -u domain\username -ppassword hostname|ip` : dump all information, resolving computer hostnames with the selected DNS server
- `ldapdomaindump --no-json -o path/to/directory -u domain\username -ppassword hostname|ip` : dump all information to the given directory without JSON output

## See also
- ldapsearch

## Categories
users, text, files

## Links
- More information: https://github.com/dirkjanm/ldapdomaindump#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/ldapdomaindump
