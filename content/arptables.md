# arptables

Manage ARP filtering rules using the `nftables` backend. Part of the `xtables-nft` suite for ARP packet filtering.

## Usage
- `sudo arptables -L` : list all ARP rules in the filter table
- `sudo arptables -A INPUT -s 192.168.0.1 -j DROP` : append a rule to drop ARP packets from a specific IP address
- `sudo arptables -D INPUT rule_number` : delete a specific rule from the INPUT chain by its rule number
- `sudo arptables -F` : flush all rules in the filter table
- `sudo arptables -P OUTPUT ACCEPT` : set the default policy of the OUTPUT chain to ACCEPT
- `sudo arptables-save >path/to/file` : save the current ARP rules to a file

## Categories
network, users, files

## Links
- More information: https://manned.org/arptables
- tldr page: https://tldr.inbrowser.app/pages/linux/arptables
