# hyprsunset

A blue light filter for Hyprland. Optionally configured via `~/.config/hypr/hyprsunset.conf`.

## Usage
- `hyprsunset` : start the hyprsunset service
- `hyprsunset -t temperature -g gamma` : start the hyprsunset service with a specified color temperature and gamma percentage
- `hyprctl hyprsunset temperature temperature` : adjust the color temperature while the hyprsunset service is running
- `hyprctl hyprsunset gamma gamma` : adjust the gamma while the hyprsunset service is running
- `hyprctl hyprsunset reset temperature` : reset the color temperature to 6000K
- `hyprctl hyprsunset reset gamma` : reset the gamma to 100%

## Categories
system, media

## Links
- More information: https://wiki.hypr.land/Hypr-Ecosystem/hyprsunset/
- tldr page: https://tldr.inbrowser.app/pages/linux/hyprsunset
