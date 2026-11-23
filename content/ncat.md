# ncat

Read, write, redirect, and encrypt data across a network. An alternative implementation of a similar utility called `netcat`/`nc`.

## Usage
- `ncat -l port >path/to/file` : listen for input on the specified port and write it to the specified file
- `ncat -lk port` : accept multiple connections and keep ncat open after they have been closed
- `ncat <path/to/file address port` : write output of specified file to the specified host on the specified port
- `ncat --ssl -k -l port` : accept multiple incoming connections on an encrypted channel evading detection of traffic content
- `ncat --ssl host port` : connect to an open `ncat` connection over SSL
- `ncat -w seconds -vz host port` : check connectivity to a remote host on a particular port with timeout

## Categories
network, files, security

## Links
- More information: https://nmap.org/ncat/guide/index.html
- tldr page: https://tldr.inbrowser.app/pages/linux/ncat
