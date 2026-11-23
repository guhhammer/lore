# evtest

Display information from input device drivers.

## Usage
- `sudo evtest` : list all detected input devices and select one for monitoring
- `sudo evtest /dev/input/eventnumber` : display events from a specific input device
- `sudo evtest --grab /dev/input/eventnumber` : grab the device exclusively, preventing other clients from receiving events
- `sudo evtest --query /dev/input/eventnumber event_type event_code` : query the state of a specific key or button on an input device

## Categories
hardware, desktop, users

## Links
- More information: https://manned.org/evtest
- tldr page: https://tldr.inbrowser.app/pages/linux/evtest
