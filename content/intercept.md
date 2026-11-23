# intercept

Read raw input events from a specified input event device and redirect it to `stdout`.

## Usage
- `sudo intercept -g /dev/input/eventX` : read and output raw input events from a given input device file (the system will not see any key presses)
- `sudo intercept /dev/input/eventX` : read and output raw input events from a given input device file (the system can see key presses and does not block other programs from reading them)

## Categories
hardware

## Links
- More information: https://gitlab.com/interception/linux/tools/-/tree/master#intercept
- tldr page: https://tldr.inbrowser.app/pages/linux/intercept
