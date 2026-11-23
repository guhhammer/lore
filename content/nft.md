# nft

Allow configuration of tables, chains, and rules provided by the Linux kernel firewall. Nftables replaces iptables.

## Usage
- `sudo nft list ruleset` : view current configuration
- `sudo nft add table inet filter` : add a new table with family "inet" and table "filter"
- `sudo nft add chain inet filter input \{ type filter hook input priority 0 \; policy accept \; \}` : add a new chain to accept all inbound traffic
- `sudo nft add rule inet filter input tcp dport \{ telnet, ssh, http, https \} accept` : add a new rule to accept several TCP ports
- `sudo nft add rule nat postrouting ip saddr 192.168.0.0/24 masquerade` : add a NAT rule to translate all traffic from the `192.168.0.0/24` subnet to the host's public IP
- `sudo nft --handle --numeric list chain family table chain` : show rule handles
- `sudo nft delete rule inet filter input handle 3` : delete a rule
- `sudo nft list ruleset >/etc/nftables.conf` : save current configuration

## Categories
users, network, databases

## Links
- More information: https://wiki.nftables.org/wiki-nftables/index.php/Main_Page
- tldr page: https://tldr.inbrowser.app/pages/linux/nft
