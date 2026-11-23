# xwininfo

Display information about windows.

## Usage
- `xwininfo` : display a cursor to select a window to display its attributes (id, name, size, position, ...)
- `xwininfo -tree -root` : display the tree of all windows
- `xwininfo -id window_id` : display the attributes of a window with a specific ID
- `xwininfo -name name` : display the attributes of a window with a specific name
- `xwininfo -tree -root | grep keyword | head -1 | perl -ne 'print $1 if /(0x[\da-f]+)/ig;'` : display the ID of a window searching by name

## See also
- xprop
- xkill

## Categories
desktop

## Links
- More information: https://www.x.org/releases/current/doc/man/man1/xwininfo.1.xhtml
- tldr page: https://tldr.inbrowser.app/pages/common/xwininfo
