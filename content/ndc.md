# ndc

Name daemon control service for name servers. If a command isn't provided, NDC will prompt for one until EOF.

## Usage
- `ndc -c channel command` : set the control channel rendezvous point
- `ndc -l localsock command` : bind the client side to a specific localsock address
- `ndc -p path/to/pidfile command` : set path to pidfile for UNIX signal control
- `ndc -d command` : enable debugging
- `ndc -q command` : enable quiet mode
- `ndc -s command` : enable nonfatal error suppression
- `ndc -t command` : enable tracing for protocol and system debugging
- `ndc /help` : list built-in commands

## Categories
system, shell

## Links
- More information: https://manned.org/ndc
- tldr page: https://tldr.inbrowser.app/pages/common/ndc
