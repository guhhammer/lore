# exiftool

Read and write meta information in files.

## Usage
- `exiftool path/to/file` : print the EXIF metadata for a given file
- `exiftool -All= path/to/file1 path/to/file2 ...` : remove all EXIF metadata from the given files
- `exiftool "-gps*=" path/to/image1 path/to/image2 ...` : remove GPS EXIF metadata from given image files
- `exiftool -All= -tagsfromfile @ -colorspacetags -orientation path/to/image1 path/to/image2 ...` : remove all EXIF metadata from the given image files, then re-add metadata for color and orientation
- `exiftool "-AllDates+=0:0:0 1:0:0" path/to/directory` : move the date at which all photos in a directory were taken 1 hour forward
- `exiftool "-AllDates-=0:0:1 2:0:0" -ext jpg` : move the date at which all JPEG photos in the current directory were taken 1 day and 2 hours backward
- `exiftool -DateTimeOriginal-=1.5 -overwrite_original` : only change the `DateTimeOriginal` field subtracting 1.5 hours, without keeping backups
- `exiftool '-filename<DateTimeOriginal' -d %Y-%m-%d_%H-%M-%S%%lc.%%e path/to/directory -r -ext jpg` : recursively rename all JPEG photos in a directory based on the `DateTimeOriginal` field

## Categories
files, media

## Links
- More information: https://exiftool.org/exiftool_pod.html
- tldr page: https://tldr.inbrowser.app/pages/common/exiftool
- Wikipedia: https://en.wikipedia.org/wiki/ExifTool
