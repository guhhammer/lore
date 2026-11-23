# scamper

Actively probes the Internet in order to analyze topology and performance. Includes some tools that start with `sc_`, for example `sc_warts2text` or `sc_ttlexp`.

## Usage
- `scamper -i 192.0.2.1` : execute the standard option (traceroute) to a destination
- `scamper -I "ping 192.0.2.1" -I "trace 192.0.2.2"` : execute two actions (ping and traceroute) on two different targets
- `scamper -c "ping -P UDP-dport -d 33434" -i 192.0.2.1 -i 192.0.2.2` : ping several hosts with UDP, use a specific port number for the first ping, and increase it for each subsequent ping
- `scamper -O warts -o path/to/output.warts -I "tracelb -P ICMP-echo -q 3 192.0.2.1"` : use the Multipath Discovery Algorithm (MDA) to determine the presence of load-balanced paths to the destination and use ICMP echo packets to sample with a maximum of three attempts, write the result to a `.warts` file
- `scamper -O warts.gz -o path/to/output.warts -I "trace -P icmp-paris 2001:db8:dead:beaf::4"` : execute a Paris traceroute with ICMP to a destination and save the result in a compressed `.warts` file
- `scamper -O warts -o path/to/output.warts -I "sniff -S 2001:db8:dead:beef::6 icmp[icmpid] == 101"` : record all ICMP packets that arrive at a specific IP address and have a specific ICMP ID in a `.warts` file

## Categories
network, files, monitoring

## Links
- More information: https://www.caida.org/catalog/software/scamper/
- tldr page: https://tldr.inbrowser.app/pages/common/scamper
