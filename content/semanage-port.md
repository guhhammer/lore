# semanage port

Manage persistent SELinux port definitions.

## Usage
- `sudo semanage port -l` : list all port labeling rules
- `sudo semanage port -l -C -n` : list all user-defined port labeling rules without headings
- `sudo semanage port -a -t ssh_port_t -p tcp 22000` : add a user-defined rule that assigns a label to a protocol-port pair
- `sudo semanage port -a -t http_port_t -p tcp 80-88` : add a user-defined rule that assigns a label to a protocol-port-range pair
- `sudo semanage port -d -p udp 11940` : delete a user-defined rule using its protocol-port pair

## Categories
network, users

## Links
- More information: https://manned.org/semanage-port
- tldr page: https://tldr.inbrowser.app/pages/linux/semanage-port
