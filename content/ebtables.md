# ebtables

Ethernet bridge frame table administration (nft-based).

## Usage
- `ebtables [-t table ] -[ACDI] chain rule specification [match extensions] [watcher extensions] target` : the general form, as the manual gives it
- `ebtables [-t table ] -P chain ACCEPT | DROP | RETURN` : another form of the same command
- `ebtables [-t table ] -F [chain]` : another form of the same command
- `ebtables [-t table ] -Z [chain]` : another form of the same command
- `ebtables [-t table ] -L [-Z] [chain] [ [--Ln] | [--Lx] ] [--Lc] [--Lmac2]` : another form of the same command
- `ebtables -A` : append a rule to the end of the selected chain
- `ebtables -D` : delete  the  specified  rule  or rules from the selected chain

## Categories
databases

## Links
- Documentation: https://wiki.nftables.org
