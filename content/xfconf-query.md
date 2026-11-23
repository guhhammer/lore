# xfconf-query

Reads and changes settings in the Xfce settings database from the terminal.

## Usage
- `xfconf-query -l` : list the channels
- `xfconf-query -c xsettings -l` : list the properties in a channel
- `xfconf-query -c xsettings -p /Net/ThemeName` : read a value
- `xfconf-query -c xsettings -p /Net/ThemeName -s Adwaita` : set a value

## Categories
databases, shell
