# bindkey

Add hotkeys to Z shell.

## Usage
- `bindkey` : list all existing hotkeys
- `bindkey "^k" kill-line` : bind a hotkey to a specific command
- `bindkey -s '^o' 'cd ..\n'` : bind a hotkey to a specific key sequence
- `bindkey -l` : list keymaps
- `bindkey -M main` : list all hotkeys in a keyMap
- `bindkey -v` : enable vi mode
- `bindkey -e` : enable emacs mode (default mode)
- `bindkey -lL main | grep -Eo 'viins|emacs'` : check which mode is active (vi or emacs)

## See also
- zle

## Categories
editors, shell

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Zsh-Line-Editor.html#Zle-Builtins
- tldr page: https://tldr.inbrowser.app/pages/common/bindkey
