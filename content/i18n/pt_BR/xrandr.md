# xrandr

Define o tamanho, orientação e/ou espelhamento das saídas para uma tela.

## Usage
- `xrandr -q` : exibe o estado atual do sistema (telas conhecidas, resoluções, ...)
- `xrandr --auto` : desativa saídas desconectadas e ativa as conectadas com as configurações padrão
- `xrandr --output DP1 --mode 1920x1080 -r 60` : altera a resolução e frequência de atualização da DisplayPort 1 para 1920x1080, 60Hz
- `xrandr --output HDMI2 --mode 1280x1024 --right-of DP1` : define a resolução do HDMI2 para 1280x1024 e o coloca à direita de DP1
- `xrandr --output VGA1 --off` : desativa a saída VGA1
- `xrandr --output LVDS1 --brightness 0.5` : define o brilho de LVDS1 como 50%
- `xrandr -d :0 -q` : mostra o estado atual de qualquer servidor X

## Links
- More information: https://www.x.org/releases/current/doc/man/man1/xrandr.1.xhtml
