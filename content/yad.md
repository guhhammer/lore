# yad

Display GTK+ dialogs from shell scripts.

## Usage
- `yad --text-info --filename path/to/file` : display the contents of a file in a text information dialog
- `yad --entry --text "message"` : open a text entry dialog and return the input to `stdout`
- `yad --file --title "title_message"` : open a file picker with a specific title
- `yad --calendar --title "title_message"` : open a date picker dialog with a specific title
- `yad --list --column "col1" --column "col2" col1_row1 col2_row1 col1_row2 col2_row2 ...` : display a list dialog with multiple columns and data
- `command | yad --progress --pulsate --auto-close --text "message"` : open a pulsating progress bar that automatically closes at 100%

## See also
- zenity

## Categories
text, desktop, files

## Links
- More information: https://manned.org/yad
- tldr page: https://tldr.inbrowser.app/pages/linux/yad
