# icoextract

Extract icons from Windows executables (`.exe`, `.dll`, `.mun`) into ICO files.

## Usage
- `icoextract path/to/input.exe path/to/output.ico` : extract the icons from an executable to an ICO file
- `icoextract -n index path/to/input.exe path/to/output.ico` : extract a specific icon by its index
- `icoextract -i resource_id path/to/input.exe path/to/output.ico` : extract an icon by its resource ID
- `icoextract -v path/to/input.exe path/to/output.ico` : run with verbose/debug logging
- `icoextract -h` : display help
- `icoextract -V` : display version

## See also
- icolist

## Categories
files, archives

## Links
- More information: https://github.com/jlu5/icoextract
- tldr page: https://tldr.inbrowser.app/pages/common/icoextract
