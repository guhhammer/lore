# ldapsearch

Query an LDAP directory.

## Usage
- `ldapsearch -D 'admin_DN' -w 'password' -h ldap_host -b base_ou 'memberOf=group1' displayName` : query an LDAP server for all items that are a member of the given group and return the object's displayName value
- `ldapsearch -D 'admin_DN' -u 'password_file' -h ldap_host -b base_ou 'memberOf=group1' displayName` : query an LDAP server with a no-newline password file for all items that are a member of the given group and return the object's displayName value
- `ldapsearch -D 'admin_DN' -w 'password' -h ldap_host -b base_ou 'memberOf=group1' -z 5 displayName` : return 5 items that match the given filter
- `ldapsearch -D 'admin_DN' -w 'password' -h ldap_host -b base_ou 'memberOf=group1' -l 7 displayName` : wait up to 7 seconds for a response
- `ldapsearch -D 'admin_DN' -w 'password' -h ldap_host -b base_ou '(!(memberOf=group1))' displayName` : invert the filter
- `ldapsearch -D 'admin_DN' -w 'password' -h ldap_host '(&(memberOf=group1)(memberOf=group2)(memberOf=group3))' "displayName"` : return all items that are part of multiple groups, returning the display name for each item
- `ldapsearch -D 'admin_DN' -w 'password' -h ldap_host '(|(memberOf=group1)(memberOf=group1)(memberOf=group3))' displayName` : return all items that are members of at least 1 of the specified groups
- `ldapsearch -D 'admin_DN' -w 'password' -h ldap_host '(&(memberOf=group1)(memberOf=group2)(!(memberOf=group3)))' displayName` : combine multiple boolean logic filters

## Categories
users, databases, files

## Links
- More information: https://docs.ldap.com/ldap-sdk/docs/tool-usages/ldapsearch.html
- tldr page: https://tldr.inbrowser.app/pages/common/ldapsearch
