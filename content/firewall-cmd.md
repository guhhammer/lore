# firewall-cmd

The firewalld client. View and adapt the runtime or permanent firewall configuration state.

## Usage
- `firewall-cmd --list-all-zones` : view all available firewall zones and rules in their runtime configuration state
- `firewall-cmd --permanent --zone block --change-interface enp1s0` : permanently move the interface into the block zone, effectively blocking all communication
- `firewall-cmd --permanent --zone public --add-service https` : permanently open the port for a service in the specified zone (like port 443 when in the `public` zone)
- `firewall-cmd --permanent --zone public --remove-service http` : permanently close the port for a service in the specified zone (like port 80 when in the `public` zone)
- `firewall-cmd --permanent --zone public --add-rich-rule 'rule family "ipv4|ipv6" forward-port port "443" protocol "udp|tcp" to-port "8443"'` : permanently forward a port for incoming packets in the specified zone (like port 443 to 8443 when entering the `public` zone)
- `firewall-cmd --reload` : reload firewalld to lose any runtime changes and force the permanent configuration to take effect immediately
- `firewall-cmd --runtime-to-permanent` : save the runtime configuration state to the permanent configuration
- `firewall-cmd --panic-on` : enable panic mode in case of Emergency. All traffic is dropped, any active connection will be terminated

## Categories
security, network, system

## Links
- More information: https://firewalld.org/documentation/man-pages/firewall-cmd
- tldr page: https://tldr.inbrowser.app/pages/linux/firewall-cmd
