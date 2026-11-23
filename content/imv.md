# imv

CLI image viewer for wayland and X11 aimed at tiling window managers. Handles multiple formats including Photoshop (PSD).

## Usage
- `imv path/to/image1.ext path/to/image2.ext ...` : view multiple images
- `imv -f path/to/image.ext` : view in fullscreen mode
- `imv -r --slideshow path/to/directory` : view images recursively from a path
- `find . -type f -name "*.svg" | imv` : open multiple images via `stdin`
- `imv -t 10 path/to/directory` : make a slideshow from a directory showing each image for 10 seconds
- `curl -Osw '%{filename_effective}\n' 'http://www.example.com/[1-10].jpg' | imv` : view multiple images from the web

## Categories
containers, desktop, files

## Links
- More information: https://sr.ht/~exec64/imv/
- tldr page: https://tldr.inbrowser.app/pages/linux/imv
