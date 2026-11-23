# kdialog

Show KDE dialog boxes from within shell scripts.

## Usage
- `kdialog --msgbox "message" "optional_detailed_message"` : open a dialog box displaying a specific message
- `kdialog --yesno "message"` : open a question dialog with a `yes` and `no` button, returning `0` and `1`, respectively
- `kdialog --warningyesnocancel "message"` : open a warning dialog with a `yes`, `no`, and `cancel` button, returning `0`, `1`, or `2` respectively
- `kdialog --inputbox "message" "optional_default_text"` : open an input dialog box and print the input to `stdout` when `OK` is pressed
- `kdialog --password "message"` : open a dialog to prompt for a specific password and print it to `stdout`
- `kdialog --combobox "message" "item1" "item2" "..."` : open a dialog containing a specific dropdown menu and print the selected item to `stdout`
- `kdialog --getopenfilename` : open a file chooser dialog and print the selected file's path to `stdout`
- `kdialog --progressbar "message"` : open a progressbar dialog and print a D-Bus reference for communication to `stdout`

## Categories
documents, desktop, shell

## Links
- More information: https://develop.kde.org/docs/administration/kdialog/
- tldr page: https://tldr.inbrowser.app/pages/linux/kdialog
