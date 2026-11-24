# zenity

Display dialogs from the command-line/shell scripts. Return user-inserted values or 1 if error.

## Usage
- `zenity --question` : display the default question dialog
- `zenity --info --text "message"` : display an info dialog displaying a message
- `zenity --forms --add-entry "name_label" --add-password "password_label" --separator ";"` : display a name/password form and output the data separated by ";" ("|" by default)
- `zenity --file-selection --directory` : display a file selection form in which the user can only select directories
- `(echo "#1"; sleep 1; echo "50"; echo "#2"; sleep 1; echo "100") | zenity --progress` : display a progress bar which updates its message every second and show a progress percent

## Categories
desktop, users, files

## Links
- More information: https://manned.org/zenity
- tldr page: https://tldr.inbrowser.app/pages/linux/zenity
- Wikipedia: https://en.wikipedia.org/wiki/Zenity
