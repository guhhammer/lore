# xdg-mime

Query and manage MIME types according to the XDG standard.

## Usage
- `xdg-mime query filetype path/to/file` : display the MIME type of a file
- `xdg-mime query default image/png` : display the default application for opening PNGs
- `xdg-mime query default $(xdg-mime query filetype path/to/file)` : display the default application for opening a specific file
- `xdg-mime default imv.desktop image/png image/jpeg` : set imv as the default application for opening PNG and JPEG images

## Categories
desktop, databases, files

## Links
- More information: https://portland.freedesktop.org/doc/xdg-mime.html
- tldr page: https://tldr.inbrowser.app/pages/linux/xdg-mime
