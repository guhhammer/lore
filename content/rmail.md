# rmail

A Mail Transfer Agent.

## Usage
- `exim4 [options] arguments ... mailq [options] arguments ... rsmtp [options] arguments ...` : the general form, as the manual gives it
- `rmail [options] arguments ... runq [options] arguments ... newaliases [options] arguments ...` : another form of the same command
- `rmail -bd` : this option runs Exim as a daemon, awaiting incoming SMTP connections
- `rmail -bpra` : this option is a combination of -bpr and -bpa
- `rmail -bpri` : this option is a combination of -bpr and -bpi
- `rmail -bpru` : this option is a combination of -bpr and -bpu

## Links
- Documentation: https://www.jetmore.org/john/code/swaks/
