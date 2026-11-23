# uinput

Intercept and write input events to a virtual keyboard device using `/dev/uinput`.

## Usage
- `uinput -p` : simulate merging the device descriptions and print the resulting YAML
- `sudo uinput -c path/to/device1.yaml path/to/device2.yaml ...` : merge YAML device description(s) to resulting virtual device
- `sudo uinput -d /dev/input/eventX /dev/input/eventY ...` : merge reference device description from device node(s) to resulting virtual device

## Categories
hardware, text

## Links
- More information: https://gitlab.com/interception/linux/tools/-/tree/master#uinput
- tldr page: https://tldr.inbrowser.app/pages/linux/uinput
