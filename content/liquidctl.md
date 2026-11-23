# liquidctl

Control liquid coolers.

## Usage
- `liquidctl list` : list available devices
- `sudo liquidctl initialize all` : initialize all supported devices
- `liquidctl status` : print the status of available liquid coolers
- `liquidctl -m string set fan|pump|... speed 50` : match a string in product name to pick a device and set its speed to a flat 50%
- `liquidctl set fan|pump|... speed 20 0 50 50 70 100` : set a gradual fan speed curve that is 0% at 20°C, 50% at 50°C, and 100% at 70°C
- `liquidctl set fan|pump|... color fixed|fading|rainbow|... 00ff00` : set the liquid cooler RGB lights (supported modes depend on the model)

## Categories
hardware

## Links
- More information: https://github.com/liquidctl/liquidctl
- tldr page: https://tldr.inbrowser.app/pages/common/liquidctl
