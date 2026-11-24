# tput

View and modify terminal settings and capabilities.

## Usage
- `tput cup row column` : move the cursor to a screen location
- `tput setaf|setab ansi_color_code` : set foreground (af) or background (ab) color
- `tput rev` : reverse text and background colors
- `tput sgr0` : reset all terminal text attributes
- `tput cols|lines|colors` : show number of columns, lines, or colors
- `tput smam|rmam` : enable or disable word wrap
- `tput civis|cnorm` : hide or show the terminal cursor
- `tput smcup|rmcup` : save or restore terminal text status (smcup also captures scroll wheel events)

## See also
- stty

## Categories
text, shell

## Links
- More information: https://manned.org/tput
- tldr page: https://tldr.inbrowser.app/pages/common/tput
- Wikipedia: https://en.wikipedia.org/wiki/Tput
