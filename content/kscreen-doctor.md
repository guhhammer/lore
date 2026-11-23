# kscreen-doctor

Change and manipulate the screen setup.

## Usage
- `kscreen-doctor -o` : show display output information
- `kscreen-doctor output.1.rotation.right` : set the rotation of a display output with an ID of 1 to the right
- `kscreen-doctor output.HDMI-2.scale.2` : set the scale of a display output with an ID of `HDMI-2` to 2 (200%)
- `kscreen-doctor output.HDMI-A-1.enable` : enable a specific HDMI display
- `kscreen-doctor output.DP-2.disable` : disable a specific DisplayPort display
- `kscreen-doctor output.HDMI-A-1.primary` : set a display as primary display
- `kscreen-doctor -h` : display help

## Categories
desktop

## Links
- More information: https://invent.kde.org/plasma/libkscreen
- tldr page: https://tldr.inbrowser.app/pages/linux/kscreen-doctor
