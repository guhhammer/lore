# grdctl

Manage GNOME Remote Desktop service.

## Usage
- `grdctl status` : list the current GRD settings
- `grdctl rdp|vnc enable` : enable Remote Desktop Protocol (RDP) or Virtual Network Computing (VNC) backend services
- `grdctl rdp set-credentials username password` : set RDP credentials
- `grdctl vnc set-password password` : set VNC password
- `grdctl rdp|vnc disable-view-only` : enable remote control of input devices
- `grdctl rdp|vnc enable-view-only` : disable remote control of input devices
- `grdctl rdp|vnc disable` : disable Remote Desktop Protocol (RDP) or Virtual Network Computing (VNC) backend services

## Categories
desktop, system, network

## Links
- More information: https://github.com/GNOME/gnome-remote-desktop/blob/main/man/grdctl.txt
- tldr page: https://tldr.inbrowser.app/pages/linux/grdctl
