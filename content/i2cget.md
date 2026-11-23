# i2cget

Read from a register of an I2C device. Note: All addresses should be specified in hexadecimal.

## Usage
- `i2cget i2cbus device_address register_address` : read from a register of an I2C device
- `i2cget -y i2cbus device_address register_address` : read from a register of an I2C device without asking for confirmation
- `i2cget i2cbus device_address register_address b|w|c|s|i` : read from a register of an I2C device using a specific mode

## See also
- i2cdetect
- i2cdump
- i2cset

## Categories
hardware

## Links
- More information: https://manned.org/i2cget
- tldr page: https://tldr.inbrowser.app/pages/linux/i2cget
