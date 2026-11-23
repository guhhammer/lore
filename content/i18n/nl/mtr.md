# mtr

Matt's Traceroute: gecombineerde traceroute- en ping-tool.

## Usage
- `mtr example.com` : traceroute naar een host en ping continu alle tussenliggende hops
- `mtr -n example.com` : schakel IP-adres en hostnaam mapping uit
- `mtr -w example.com` : genereer output na het pingen van elke hop 10 keer
- `mtr -4 example.com` : forceer IPv4 of IPv6
- `mtr -i 10 example.com` : wacht een gegeven tijd (in seconden) voordat een volgend pakket naar dezelfde hop wordt verstuurd
- `mtr -z example.com` : toon het Autonomous System Number (ASN) voor elke hop
- `mtr -b example.com` : toon zowel het IP-adres als de reverse DNS-naam

## Links
- More information: https://manned.org/mtr
