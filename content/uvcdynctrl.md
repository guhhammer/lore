# uvcdynctrl

Manage dynamic controls in uvcvideo.

## Usage
- `uvcdynctrl -l` : list all available cameras
- `uvcdynctrl -d device_name` : use a specific device (defaults to `video0`)
- `uvcdynctrl -c` : list available controls
- `uvcdynctrl -s control_name value` : set a new control value (for negative values, use `-- -value`)
- `uvcdynctrl -g control_name` : get the current control value
- `uvcdynctrl -W filename` : save the state of the current controls to a file
- `uvcdynctrl -L filename` : load the state of the controls from a file

## Links
- More information: https://manned.org/uvcdynctrl
- tldr page: https://tldr.inbrowser.app/pages/linux/uvcdynctrl
