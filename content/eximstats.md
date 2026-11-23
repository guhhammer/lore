# eximstats

Generates statistics from Exim mainlog or syslog files.

## Usage
- `eximstats [Output] [Options] mainlog1 mainlog2 ...` : the general form, as the manual gives it
- `eximstats mainlog.sun > report.sun.txt` : another form of the same command
- `eximstats mainlog.mon > report.mon.txt` : another form of the same command
- `eximstats mainlog.tue > report.tue.txt` : another form of the same command
- `eximstats mainlog.wed > report.web.txt` : another form of the same command
- `eximstats -txt` : output the results in plain text to STDOUT
- `eximstats -txt=filename` : output the results in plain text

## Categories
monitoring

## Links
- Documentation: https://www.exim.org/
