# redshift

Adjust the color temperature of a screen according to its surroundings. Note: Redshift does not support Wayland.

## Usage
- `redshift -t 5700:3600` : turn on Redshift with temperature set to 5700k during the day and 3600k at night
- `redshift -l latitude:longitude` : turn on Redshift with a manually specified custom location
- `redshift -b 0.7:0.4` : turn on Redshift with screen brightness set to 70% during the day and 40% at night
- `redshift -g red:green:blue` : turn on Redshift with custom gamma levels (between 0 and 1)
- `redshift -PO temperature` : purge existing temperature changes and set a constant unchanging color temperature in One-shot mode

## See also
- gammastep

## Categories
desktop, media

## Links
- More information: https://manned.org/redshift
- tldr page: https://tldr.inbrowser.app/pages/common/redshift
