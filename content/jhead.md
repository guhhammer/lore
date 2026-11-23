# jhead

Image timestamp and EXIF data manipulation.

## Usage
- `jhead path/to/image.jpg` : show all EXIF data
- `jhead -ft path/to/image.jpg` : set the file's date and time to the EXIF create date (file creation date will be changed)
- `jhead -dsft path/to/image.jpg` : set the EXIF time to the file's date and time (EXIF data will be changed)
- `jhead -n%Y_%m_%d-%H_%M_%S *.jpg` : rename all JPEG files based on the EXIF create date to `YYYY_MM_DD-HH_MM_SS.jpg`
- `jhead -autorot *.jpg` : rotate losslessly all JPEG images by 90, 180, or 270 degrees based on the EXIF orientation tag
- `jhead -ta-1:00:00 *.jpg` : update all EXIF timestamps (Format: +- hour:minute:seconds) (example: forgot to change the camera's time zone - removing 1 hour from timestamps)
- `jhead -purejpg path/to/image.jpg` : remove all EXIF data (including thumbnails)

## Categories
containers, files, media

## Links
- More information: https://www.sentex.net/~mwandel/jhead/usage.html
- tldr page: https://tldr.inbrowser.app/pages/linux/jhead
