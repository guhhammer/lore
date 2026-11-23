# zmap

Fast, open-source network scanner for Internet-wide surveys.

## Usage
- `sudo zmap -p port subnet` : scan a subnet or full IPv4 space for a specific TCP port (default: 80)
- `sudo zmap -p port1,port2-port3,... subnet` : scan specific ports or port ranges across a subnet
- `sudo zmap -o path/to/output_file.csv -f "saddr,daddr,sport,dport" subnet` : output results to a CSV file with custom fields
- `sudo zmap -r packets_per_second subnet` : limit the scan rate to a specific number of packets per second
- `zmap -d subnet` : simulate running zmap without sending packets
- `sudo zmap -b path/to/blocklist.txt subnet` : exclude subnets using a blocklist file in CIDR notation
- `sudo zmap -S source_ip subnet` : set a specific source IP for scan packets
- `sudo zmap -p port1,port2-port3 -n 1000 subnet` : cap the number/percentage of targets to probe (e.g. 1000 IP/port pairs)

## See also
- hping3
- masscan
- naabu
- nmap
- rustscan

## Categories
network, users, files

## Links
- More information: https://manned.org/zmap
- tldr page: https://tldr.inbrowser.app/pages/common/zmap
