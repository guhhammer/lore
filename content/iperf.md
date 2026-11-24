# iperf

Measure network bandwidth between computers.

## Usage
- `iperf -s` : run on server
- `iperf -u -s -p 5001` : run on server using UDP mode and set server port to listen on 5001
- `iperf -c server_address` : run on client
- `iperf -c server_address -i 2` : run on client every 2 seconds
- `iperf -c server_address -P 5` : run on client with 5 parallel threads
- `iperf -u -c server_address -p 5001` : run on client using UDP mode

## Categories
network

## Links
- More information: https://iperf.fr/iperf-doc.php
- tldr page: https://tldr.inbrowser.app/pages/common/iperf
- Wikipedia: https://en.wikipedia.org/wiki/Iperf
