# xrandr

Stel de grootte, oriëntatie en/of reflectie van de outputs voor een scherm in.

## Usage
- `xrandr -q` : toon de huidige status van het systeem (bekende schermen, resoluties, ...)
- `xrandr --auto` : schakel losgekoppelde outputs uit en schakel verbonden outputs aan met de standaardinstellingen
- `xrandr --output DP1 --mode 1920x1080 -r 60` : wijzig de resolutie en updatefrequentie van DisplayPort 1 naar 1920x1080, 60Hz
- `xrandr --output HDMI2 --mode 1280x1024 --right-of DP1` : stel de resolutie van HDMI2 in op 1280x1024 en plaats deze rechts van DP1
- `xrandr --output VGA1 --off` : schakel de VGA1 output uit
- `xrandr --output LVDS1 --brightness 0.5` : stel de helderheid voor LVDS1 in op 50%
- `xrandr -d :0 -q` : toon de huidige status van een X-server

## Links
- More information: https://www.x.org/releases/current/doc/man/man1/xrandr.1.xhtml
