# randpkt

Random packet generator.

## Usage
- `randpkt [ -b <maxbytes> ] [ -c <count> ] [ -F <file format> ] [ -r ] [ -t <type> ] <filename>` : the general form, as the manual gives it
- `randpkt -h|--help` : another form of the same command
- `randpkt -v|--version` : another form of the same command
- `randpkt -b <maxbytes>` : default 5000
- `randpkt -c <count>` : default 1000
- `randpkt -r` : the packet type is determined randomly for each packet
- `randpkt -t <type>` : default Ethernet II frame

## Categories
network
