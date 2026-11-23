# i2cset

Set the value of a register of an I2C device. Note: All addresses should be specified in hexadecimal.

## Usage
- `i2cset i2cbus device_address register_address value` : write to a register of an I2C device
- `i2cset -y i2cbus device_address register_address value` : write to a register of an I2C device without asking for confirmation
- `i2cset i2cbus device_address register_address value b|w|c|s|i` : write to a register of an I2C device using a specific mode

## See also
- i2cdetect
- i2cdump
- i2cget

## Categories
hardware

## Links
- More information: https://manned.org/i2cset
- tldr page: https://tldr.inbrowser.app/pages/linux/i2cset
