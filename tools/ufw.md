# ufw

ufw is the simple firewall of Ubuntu and Debian: it writes iptables or nftables rules from short commands.

## Tasks
### Turn it on
- `sudo ufw allow OpenSSH` : allow ssh first, so you do not lock yourself out
- `sudo ufw enable` : turn the firewall on
- `sudo ufw disable` : turn it off
- `sudo ufw status verbose` : state and default policies
- `sudo ufw status numbered` : rules with numbers
- `sudo ufw reset` : remove every rule and turn it off (deletes the rules)
### Defaults
- `sudo ufw default deny incoming` : block what arrives unless allowed
- `sudo ufw default allow outgoing` : let your machine reach out
### Allow
- `sudo ufw allow 22/tcp` : open a port
- `sudo ufw allow 80,443/tcp` : open several ports
- `sudo ufw allow 60000:61000/tcp` : open a range
- `sudo ufw allow from 203.0.113.5` : allow one address everywhere
- `sudo ufw allow from 192.168.1.0/24 to any port 22 proto tcp` : one network to one port
- `sudo ufw allow in on NAME to any port 8080` : only on one interface
- `sudo ufw allow "Nginx Full"` : an app profile
### Block and remove
- `sudo ufw deny 3306/tcp` : block a port
- `sudo ufw deny from 203.0.113.5` : block an address
- `sudo ufw limit 22/tcp` : allow but slow down repeated connections
- `sudo ufw delete allow 80/tcp` : remove a rule by what it says
- `sudo ufw delete 3` : remove rule number 3
### Look
- `sudo ufw app list` : known application profiles
- `sudo ufw logging medium` : set the log level
- `sudo ufw reload` : reload the rules
- `sudo tail -f /var/log/ufw.log` : follow the blocked traffic

## Notes
- Allow ssh before `ufw enable` on a remote machine, or you cut yourself off.
- Rules apply in order and the first match wins: check `status numbered`.
- Docker's published ports bypass ufw: do not rely on it for containers.
- ufw only shows the rules it made, not other iptables rules.
- For Fedora and Red Hat systems see firewalld.

## See also
- firewalld
- iptables
- nft
- ss

## Links
- Ubuntu guide: https://help.ubuntu.com/community/UFW
- Manual: https://manpages.ubuntu.com/manpages/noble/man8/ufw.8.html

## Categories
security network
