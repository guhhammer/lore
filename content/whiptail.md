# whiptail

Display text-based dialog boxes from shell scripts.

## Usage
- `whiptail --title "title" --msgbox "message" height_in_chars width_in_chars` : display a simple message
- `whiptail --title "title" --yesno "message" height_in_chars width_in_chars` : display a boolean choice, returning the result through the exit code
- `whiptail --title "title" --yes-button "text" --no-button "text" --yesno "message" height_in_chars width_in_chars` : customize the text on the yes/no buttons
- `result_variable_name="$(whiptail --title "title" --inputbox "message" height_in_chars width_in_chars default_text 3>&1 1>&2 2>&3)"` : display a text input box
- `result_variable_name="$(whiptail --title "title" --passwordbox "message" height_in_chars width_in_chars 3>&1 1>&2 2>&3)"` : display a password input box
- `result_variable_name=$(whiptail --title "title" --menu "message" height_in_chars width_in_chars menu_display_height "value_1" "display_text_1" "value_2" "display_text_2" ... 3>&1 1>&2 2>&3)` : display a multiple-choice menu

## See also
- dialog
- gum

## Categories
desktop, text, shell

## Links
- More information: https://manned.org/whiptail
- tldr page: https://tldr.inbrowser.app/pages/linux/whiptail
