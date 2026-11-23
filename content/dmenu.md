# dmenu

Dynamic menu. Create a menu from a text input with each item on a new line.

## Usage
- `ls | dmenu` : display a menu of the output of the `ls` command
- `echo -e "red\ngreen\nblue" | dmenu` : display a menu with custom items separated by a new line (`\n`)
- `echo -e "red\ngreen\nblue" | dmenu >color.txt` : let the user choose between multiple items and save the selected one to a file
- `ls | dmenu -m 1` : launch dmenu on a specific monitor
- `ls | dmenu -b` : display dmenu at the bottom of the screen

## Categories
text, desktop

## Links
- More information: https://manned.org/dmenu
- tldr page: https://tldr.inbrowser.app/pages/linux/dmenu
