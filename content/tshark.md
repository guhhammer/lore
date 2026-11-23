# tshark

Packet analysis tool, CLI version of Wireshark.

## Usage
- `tshark` : monitor packets from the default interface
- `tshark -f 'udp port 53'` : only capture packets matching a specific capture filter
- `tshark -Y 'http.request.method == "GET"'` : only show packets matching a specific output filter
- `tshark -d tcp.port==8888,http` : decode a TCP port using a specific protocol (e.g. HTTP)
- `tshark -T json|text|ps|...` : specify the format of captured output
- `tshark -T fields|ek|json|pdml -e http.request.method -e ip.src` : select specific fields to output
- `tshark -w path/to/file` : write captured packet to a file
- `tshark -r path/to/file.pcap` : analyze packets from a file

## Categories
network, files, text

## Links
- More information: https://tshark.dev/#sitemap-in-tshark---help
- tldr page: https://tldr.inbrowser.app/pages/linux/tshark
