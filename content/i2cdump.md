# i2cdump

Dump I2C device registers. Note: All addresses should be specified in hexadecimal.

## Usage
- `i2cdump i2cbus device_address` : dump all registers of an I2C device
- `i2cdump -y i2cbus device_address` : dump all registers of an I2C device without asking for confirmation
- `i2cdump i2cbus device_address b|w|c|s|i` : dump all registers of an I2C device using a specific mode
- `i2cdump -r start-end i2cbus device_address` : dump registers from `start` to `end` of an I2C device

## See also
- i2cdetect
- i2cget
- i2cset

## Categories
hardware

## Links
- More information: https://manned.org/i2cdump
- tldr page: https://tldr.inbrowser.app/pages/linux/i2cdump
