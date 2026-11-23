# ddcutil

Control the settings of connected displays via DDC/CI. This command requires the kernel module `i2c-dev` to be loaded.

## Usage
- `ddcutil det` : list all compatible displays
- `ddcutil cap` : query the first compatible display for capabilities
- `ddcutil -d 1 set 10 50` : change the brightness (option `10`) of display 1 to 50%
- `ddcutil -d 1 set 12 + 5` : increase the contrast (option `12`) of display 1 by 5%
- `ddcutil -d 1 set 60 0x0f` : change the display source (option `60`) of a display
- `ddcutil -d 1 get ALL` : read the settings of display 1

## See also
- modprobe

## Categories
desktop, system, development

## Links
- More information: https://www.ddcutil.com/commands/
- tldr page: https://tldr.inbrowser.app/pages/linux/ddcutil
