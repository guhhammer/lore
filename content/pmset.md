# pmset

Configure macOS power management settings, as one might do in System Preferences > Energy Saver. Commands that modify settings must begin with `sudo`.

## Usage
- `pmset -g` : display the current power management settings
- `pmset -g batt` : display the current power source and battery levels
- `pmset displaysleepnow` : put display to sleep immediately
- `sudo pmset -c displaysleep 0` : set display to never sleep when on charger power
- `sudo pmset -b displaysleep 15` : set display to sleep after 15 minutes when on battery power
- `sudo pmset repeat wake MTWRF 09:00:00` : schedule computer to automatically wake up every weekday at 9 AM
- `sudo pmset -a displaysleep 10 disksleep 10 sleep 30 womp 1` : restore to system defaults

## Categories
users, desktop, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/pmset.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/pmset
