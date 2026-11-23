# xmodmap

Utility for modifying keymaps and pointer button mappings in X.

## Usage
- `xmodmap -e 'pointer = 3 2 1'` : swap `<LeftClick>` and `<RightClick>` on the pointer
- `xmodmap -e 'keycode keycode = keyname'` : reassign a key on the keyboard to another key
- `xmodmap -e 'keycode keycode ='` : disable a key on the keyboard
- `xmodmap path/to/file` : execute all xmodmap expressions in the specified file

## Categories
files

## Links
- More information: https://manned.org/xmodmap
- tldr page: https://tldr.inbrowser.app/pages/linux/xmodmap
