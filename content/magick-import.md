# magick import

Capture some or all of an X server screen and save the image to a file.

## Usage
- `magick import -window root path/to/output.ps` : capture the entire X server screen into a PostScript file
- `magick import -window root -display remote_host:screen.display path/to/output.png` : capture contents of a remote X server screen into a PNG image
- `magick import -window window_id path/to/output.jpg` : capture a specific window given its ID as displayed by `xwininfo` into a JPEG image

## Categories
desktop, files, containers

## Links
- More information: https://imagemagick.org/script/import.php
- tldr page: https://tldr.inbrowser.app/pages/common/magick-import
