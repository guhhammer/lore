# mailq

A Mail Transfer Agent.

## Usage
- `exim4 [options] arguments ...` : the general form, as the manual gives it
- `mailq -bd` : this option runs Exim as a daemon, awaiting incoming SMTP connections
- `mailq -bpra` : this option is a combination of -bpr and -bpa
- `mailq -bpri` : this option is a combination of -bpr and -bpi
- `mailq -bpru` : this option is a combination of -bpr and -bpu

## Links
- Documentation: https://www.jetmore.org/john/code/swaks/
