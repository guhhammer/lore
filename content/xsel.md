# xsel

X11 selection and clipboard manipulation tool.

## Usage
- `echo 123 | xsel -ib` : use a command's output as input of the clipboard (equivalent to `<Ctrl c>`)
- `cat path/to/file | xsel -ib` : use the contents of a file as input of the clipboard
- `xsel -ob` : output the clipboard's contents into the terminal (equivalent to `<Ctrl v>`)
- `xsel -ob >path/to/file` : output the clipboard's contents into a file
- `xsel -cb` : clear the clipboard
- `xsel -op` : output the X11 primary selection's contents into the terminal (equivalent to a mouse `<MiddleClick>`)

## Categories
desktop, files

## Links
- More information: https://manned.org/xsel
- tldr page: https://tldr.inbrowser.app/pages/linux/xsel
