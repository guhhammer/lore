# firewalld

firewalld is the firewall manager of Fedora, RHEL and derivatives. Rules belong to zones; firewall-cmd changes them.

## Tasks
### Look
- `sudo firewall-cmd --state` : is it running
- `sudo firewall-cmd --get-active-zones` : zones in use and their interfaces
- `sudo firewall-cmd --list-all` : everything allowed in the default zone
- `sudo firewall-cmd --list-all --zone=NAME` : the same for one zone
- `sudo firewall-cmd --get-services` : names of known services
### Allow
- `sudo firewall-cmd --add-service=http` : allow a service now (until reload or reboot)
- `sudo firewall-cmd --permanent --add-service=http` : allow it for good
- `sudo firewall-cmd --permanent --add-port=8080/tcp` : open a port for good
- `sudo firewall-cmd --permanent --add-port=60000-61000/udp` : open a range
- `sudo firewall-cmd --permanent --add-source=192.168.1.0/24` : trust a network
- `sudo firewall-cmd --permanent --add-rich-rule='rule family="ipv4" source address="203.0.113.5" port port="22" protocol="tcp" accept'` : allow one address to one port
### Block and remove
- `sudo firewall-cmd --permanent --remove-service=http` : close a service
- `sudo firewall-cmd --permanent --remove-port=8080/tcp` : close a port
- `sudo firewall-cmd --permanent --add-rich-rule='rule family="ipv4" source address="203.0.113.5" reject'` : block an address
### Apply
- `sudo firewall-cmd --reload` : load the permanent rules
- `sudo firewall-cmd --runtime-to-permanent` : save what you did without `--permanent`
- `sudo firewall-cmd --panic-on` : drop all traffic (cuts ssh too)
- `sudo firewall-cmd --panic-off` : end panic mode
### Zones and interfaces
- `sudo firewall-cmd --get-default-zone` : the default zone
- `sudo firewall-cmd --set-default-zone=public` : change it
- `sudo firewall-cmd --permanent --zone=internal --change-interface=NAME` : put an interface in a zone
- `sudo firewall-cmd --permanent --add-masquerade` : turn on NAT for the zone
### Service control
- `sudo systemctl enable --now firewalld` : start it now and at boot

## Notes
- Without `--permanent` a rule disappears at reload or reboot; with it, the rule only works after `--reload`.
- Do not use `--panic-on` over ssh.
- Keep ssh allowed before changing zones on a remote machine.
- Debian and Ubuntu users usually have ufw instead.

## See also
- ufw
- iptables
- nft
- systemd

## Links
- Docs: https://firewalld.org/documentation/
- Manual: https://firewalld.org/documentation/man-pages/firewall-cmd.html

## Categories
security network
