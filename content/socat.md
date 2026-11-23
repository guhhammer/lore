# socat

Multipurpose relay (SOcket CAT).

## Usage
- `sudo socat - TCP-LISTEN:8080,fork` : listen to a port, wait for an incoming connection, and transfer data to STDIO
- `sudo socat OPENSSL-LISTEN:4433,reuseaddr,cert=./cert.pem,cafile=./ca.cert.pem,key=./key.pem,verify=0 STDOUT` : listen on a port using SSL and print to `stdout`
- `sudo socat - TCP4:www.example.com:80` : create a connection to a host and port, transfer data in STDIO to connected host
- `sudo socat TCP-LISTEN:80,fork TCP4:www.example.com:80` : forward incoming data of a local port to another host and port
- `echo "Hello Multicast" | socat - UDP4-DATAGRAM:224.0.0.1:5000` : send data with multicast routing scheme
- `socat - UDP4-RECVFROM:5000` : receive data from a multicast

## Categories
network, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/socat
