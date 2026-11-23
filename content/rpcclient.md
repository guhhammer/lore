# rpcclient

MS-RPC client tool (part of the samba suite).

## Usage
- `rpcclient -U domain\username%password ip_address` : connect to a remote host
- `rpcclient -U username -W domain -N ip_address` : connect to a remote host on a domain without a password
- `rpcclient -U domain\username --pw-nt-hash ip_address` : connect to a remote host, passing the password hash
- `rpcclient -U domain\username%password -c semicolon_separated_commands ip_address` : execute shell commands on a remote host
- `enumdomusers` : [Interactive] Display domain users
- `enumprivs` : [Interactive] Display privileges
- `queryuser username|rid` : [Interactive] Display information about a specific user
- `createdomuser username` : [Interactive] Create a new user in the domain

## Categories
users, network, desktop

## Links
- More information: https://www.samba.org/samba/docs/current/man-html/rpcclient.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/rpcclient
