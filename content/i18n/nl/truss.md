# truss

Troubleshooting tool voor het traceren van system calls. SunOS equivalent van `strace`.

## Usage
- `truss -f programma` : start het traceren van een programma door het uit te voeren, en volg alle child processes
- `truss -p proces_id` : start het traceren van een specifiek proces aan de hand van de PID
- `truss -a -e programma` : start het traceren van een programma door het uit te voeren, en toon alle argumenten en omgevingsvariabelen
- `truss -c -p proces_id` : tel tijd, oproepen en fouten voor elke system call en geef een overzicht bij de beëindiging van de applicatie
- `truss -p proces_id -t system_call_naam` : traceer een proces en filter de uitvoer op system call

## Links
- More information: https://www.unix.com/man-page/sunos/1/truss
