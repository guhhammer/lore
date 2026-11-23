# iperf3

Traffic generator for testing network bandwidth.

## Usage
- `iperf3 -s` : run iperf3 as a server
- `iperf3 -s -p port` : run an iperf3 server on a specific port
- `iperf3 -c server_ip` : start bandwidth test
- `iperf3 -c server_ip -P streams` : run iperf3 in multiple parallel streams
- `iperf3 -c server_ip -R` : run the test in reverse direction (server sends data to client)
- `iperf3 -c server_ip -t seconds` : set the duration of the test

## Categories
network, development

## Links
- More information: https://iperf.fr/iperf-doc.php
- tldr page: https://tldr.inbrowser.app/pages/common/iperf3
