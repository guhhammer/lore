# scanimage

Scan images with the Scanner Access Now Easy API.

## Usage
- `scanimage -L` : list available scanners to ensure the target device is connected and recognized
- `scanimage --format pnm|tiff|png|jpeg|pdf|... >path/to/new_image` : scan an image and save it to a file
- `scanimage -d device_name >path/to/new_image` : specify the device to scan from
- `scanimage --resolution 300 >path/to/new_image` : specify resolution for the scanned image (default resolution is 75dpi)

## Categories
media, containers, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/scanimage
