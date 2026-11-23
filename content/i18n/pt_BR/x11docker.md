# x11docker

Executar aplicativos de GUI e interfaces de desktop seguramente em contêineres do Docker.

## Usage
- `x11docker -p --share $HOME/Videos jess/vlc` : inicia o VLC em um contêiner
- `x11docker -d x11docker/xfce` : inicia o Xfce em uma janela
- `x11docker -d -g --init=systemd x11docker/gnome` : inicia o GNOME em uma janela
- `x11docker -d -g --init=systemd x11docker/kde-plasma` : inicia o KDE Plasma em uma janela
- `x11docker --help` : mostra ajuda

## Links
- More information: https://github.com/mviereck/x11docker#terminal-syntax
