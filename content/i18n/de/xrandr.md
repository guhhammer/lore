# xrandr

Setzt die Auflösung, Orientierung und/oder Reflektion eines Bildschirmausgangs.

## Usage
- `xrandr -q` : zeige den momentanen Systemzustand an (erkannte Bildschirme, Auflösungen, ...)
- `xrandr --auto` : deaktiviere nicht mehr verbundene Ausgangsgeräte und aktiviere verbundene Ausgänge mit Standardeinstellungen
- `xrandr --output DP1 --mode 1920x1080 --rate 60` : ändere die Auflösung und Bildfrequenz von DisplayPort 1 zu 1920x1080, 60Hz
- `xrandr --output HDMI2 --mode 1280x1024 --right-of DP1` : setze die Auflösung von HDMI auf 1280x1024 und platziere HDMI1 rechts von DP1
- `xrandr --output VGA1 --off` : deaktiviere den Ausgang von VGA1
- `xrandr --output LVDS1 --brightness 0.5` : setze die Bildschirmhelligkeit von LVDS1 auf 50%

## Links
- More information: https://www.x.org/releases/current/doc/man/man1/xrandr.1.xhtml
