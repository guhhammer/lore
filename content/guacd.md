# guacd

Apache Guacamole proxy daemon. Support loader for client plugins to interface between the Guacamole protocol and any arbitrary remote desktop protocol (e.g. RDP, VNC, Other).

## Usage
- `guacd -b 127.0.0.1 -l 4823` : bind to a specific port on localhost
- `guacd -f -L debug` : start in debug mode, keeping the process in the foreground
- `guacd -C my-cert.crt -K my-key.pem` : start with TLS support
- `guacd -p path/to/file.pid` : write the PID to a file

## Categories
network, desktop, files

## Links
- More information: https://manned.org/guacd
- tldr page: https://tldr.inbrowser.app/pages/common/guacd
