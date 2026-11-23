# x11docker

Securely run GUI applications and desktop UIs in Docker containers.

## Usage
- `x11docker -p --share $HOME/Videos jess/vlc` : launch VLC in a container
- `x11docker -d x11docker/xfce` : launch Xfce in a window
- `x11docker -d -g --init=systemd x11docker/gnome` : launch GNOME in a window
- `x11docker -d -g --init=systemd x11docker/kde-plasma` : launch KDE Plasma in a window
- `x11docker --help` : display help

## See also
- xephyr

## Categories
desktop, containers

## Links
- More information: https://github.com/mviereck/x11docker#terminal-syntax
- tldr page: https://tldr.inbrowser.app/pages/common/x11docker
