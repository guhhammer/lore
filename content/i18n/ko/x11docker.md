# x11docker

Docker 컨테이너에서 GUI 애플리케이션 및 데스크톱 UI를 안전하게 실행.

## Usage
- `x11docker -p --share $HOME/Videos jess/vlc` : 컨테이너에서 VLC 실행
- `x11docker -d x11docker/xfce` : 창에서 Xfce 실행
- `x11docker -d -g --init=systemd x11docker/gnome` : 창에서 GNOME 실행
- `x11docker -d -g --init=systemd x11docker/kde-plasma` : 창에서 KDE Plasma 실행
- `x11docker --help` : 도움말 표시

## Links
- More information: https://github.com/mviereck/x11docker#terminal-syntax
