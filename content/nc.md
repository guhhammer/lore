# nc

Redirect I/O into a network stream through this versatile tool.

## Usage
- `nc <filename -l -p port` : start a listener on the specified TCP port and send a file into it
- `nc host port >received_filename` : connect to a target listener on the specified port and receive a file from it
- `nc -v -z -w timeout_in_seconds host start_port-end_port` : scan the open TCP ports of a specified host
- `nc -l -p port -e shell_executable` : start a listener on the specified TCP port and provide your local shell access to the connected party (this is dangerous and can be abused)
- `nc host port -e shell_executable` : connect to a target listener and provide your local shell access to the remote party (this is dangerous and can be abused)
- `nc -l -p local_port | nc host remote_port` : act as a proxy and forward data from a local TCP port to the given remote host
- `echo -e "GET / HTTP/1.1\nHost: host\n\n" | nc host 80` : send an HTTP GET request

## Categories
network

## Links
- More information: https://manned.org/nc
- tldr page: https://tldr.inbrowser.app/pages/common/nc
- Wikipedia: https://en.wikipedia.org/wiki/Netcat
