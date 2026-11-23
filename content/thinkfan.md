# thinkfan

A simple fan control program.

## Usage
- `thinkfan [-hnqDd] [-b BIAS] [-c CONFIG] [-s SECONDS] [-p [DELAY]]` : the general form, as the manual gives it
- `thinkfan -h` : show a short help message
- `thinkfan -s SECONDS` : maximum seconds between temperature updates (default: 5)
- `thinkfan -c FILE` : load a different configuration file
- `thinkfan -n` : do not become a daemon and log to terminal instead of syslog

## Links
- Documentation: https://www.kernel.org/doc/html/latest/hwmon/sysfs-interface.html
