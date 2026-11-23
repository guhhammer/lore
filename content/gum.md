# gum

Make glamorous shell scripts.

## Usage
- `gum choose "option1" "option2" ...` : interactively pick a specific option to print to `stdout`
- `gum input --placeholder "value"` : open an interactive prompt for the user to input a string with a specific placeholder
- `gum confirm "Continue?" --default=false --affirmative "Yes" --negative "No" && echo "Yes selected" || echo "No selected"` : open an interactive confirmation prompt and exit with either `<0>` or `<1>`
- `gum spin -s dot|line|minidot|jump|pulse|points|globe|moon|monkey|meter|hamburger --title "loading..." command` : show a spinner while a command is taking place with text alongside
- `gum format -t emoji ":smile: :heart: hello"` : format text to include emojis
- `gum write >path/to/file` : interactively prompt for multi-line text and write the input to a file

## See also
- whiptail
- dialog

## Categories
shell, text, files

## Links
- More information: https://github.com/charmbracelet/gum#tutorial
- tldr page: https://tldr.inbrowser.app/pages/common/gum
