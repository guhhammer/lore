# kscreenctl

Manage display outputs on KDE Plasma.

## Usage
- `kscreenctl list-outputs` : list all connected outputs
- `kscreenctl identify` : show an on-screen label on each monitor to identify connector names
- `kscreenctl DP-1` : show information about a specific output (or use `active-output` / `primary-output`)
- `kscreenctl DP-1 set-mode 1920x1080@60` : set the resolution and refresh rate of an output
- `kscreenctl DP-1 set-scale 200%` : set the scale of an output to 200%
- `kscreenctl HDMI-A-1 right-of DP-1` : place an output to the right of another
- `kscreenctl HDMI-A-1 set-enabled false` : disable a specific output
- `kscreenctl off` : turn all outputs off through Display Power Management Signaling

## See also
- kscreen-doctor
- kscreen-console

## Categories
desktop

## Links
- More information: https://invent.kde.org/plasma/kscreen
- tldr page: https://tldr.inbrowser.app/pages/linux/kscreenctl
