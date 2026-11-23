# alsactl

Advanced controls for ALSA soundcard driver.

## Usage
- `alsactl [options] [store|restore|init] <card # or id or device>` : the general form, as the manual gives it
- `alsactl monitor <card # or id>` : another form of the same command
- `alsactl info <card # or id>` : another form of the same command
- `alsactl [clean] <card # or id or device> [[control identifiers]]` : another form of the same command
- `alsactl -h` : help: show available flags and commands
- `alsactl -d` : use debug mode: a bit more verbose
- `alsactl -v` : print alsactl version number
