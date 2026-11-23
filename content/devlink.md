# devlink

Devlink tool.

## Usage
- `devlink [ OPTIONS ] { dev|port|monitor|sb|resource|region|health|trap } { COMMAND | help }` : the general form, as the manual gives it
- `devlink [ -force ] -batch filename` : another form of the same command
- `devlink -V` : print the version of the devlink utility and exit
- `devlink -b` : read commands from provided file or standard input and invoke them
- `devlink -j` : generate JSON output
- `devlink -p` : when combined with -j generate a pretty JSON output
