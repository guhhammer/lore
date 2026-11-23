# xprop

Display window and font properties in an X server.

## Usage
- `xprop -root WM_NAME` : display the name of the root window
- `xprop -name "window_name" WM_HINTS` : display the window manager hints for a window
- `xprop -font "font_name" POINT_SIZE` : display the point size of a font
- `xprop -id 0x200007` : display all the properties of the window with the ID 0x200007
- `xprop -spy` : monitor a window for changes

## Categories
desktop, media

## Links
- More information: https://manned.org/xprop
- tldr page: https://tldr.inbrowser.app/pages/common/xprop
