# rsmtp

A Mail Transfer Agent.

## Usage
- `exim4 [options] arguments ... mailq [options] arguments ...` : the general form, as the manual gives it
- `rsmtp -bd` : this option runs Exim as a daemon, awaiting incoming SMTP connections
- `rsmtp -bpra` : this option is a combination of -bpr and -bpa
- `rsmtp -bpri` : this option is a combination of -bpr and -bpi
- `rsmtp -bpru` : this option is a combination of -bpr and -bpu

## Links
- Documentation: https://www.jetmore.org/john/code/swaks/
