# sensors

Report sensors information.

## Usage
- `sensors` : show the current readings of all sensor chips
- `sensors -f` : show temperatures in degrees Fahrenheit
- `sensors chip_name` : show the readings of a specific sensor chip (run `sensors` to find chip name, e.g. coretemp-isa-0000)
- `sensors -j` : output sensor data in JSON format
- `sensors -u` : show raw sensor output for scripting or debugging purposes
- `sensors -c path/to/sensors.conf` : show sensor readings using a specific configuration file
- `sensors --bus-list` : list all detected hardware communication buses (like I2C or ISA)

## See also
- btm
- btop

## Categories
hardware

## Links
- More information: https://manned.org/sensors
- tldr page: https://tldr.inbrowser.app/pages/linux/sensors
