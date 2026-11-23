# wlsunset

Adjust the color temperature in Wayland compositors depending on the sunset/sunrise times.

## Usage
- `wlsunset -l latitude -L longitude` : automatically compute sunset/sunrise times based on the specified location
- `wlsunset -s sunset_time -S sunrise_time` : manually set the sunset/sunrise times (time format: `HH:MM`)
- `wlsunset -T high_temp -t low_temp` : set the high and low temperatures (default: high - 6500, low - 4000)
- `wlsunset -d n -s sunset_time -S sunrise_time` : set the animation time to `n` seconds (only applicable when using manual sunset/sunrise times)
- `wlsunset -g gamma_value` : set the gamma (default: 1.0)

## Categories
desktop, media

## Links
- More information: https://manned.org/wlsunset
- tldr page: https://tldr.inbrowser.app/pages/common/wlsunset
