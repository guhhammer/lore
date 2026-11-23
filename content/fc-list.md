# fc-list

List available fonts installed on the system.

## Usage
- `fc-list` : return a list of installed fonts
- `fc-list | grep 'DejaVu Serif'` : return a list of installed fonts with given name
- `fc-list | wc -l` : return the number of installed fonts
- `fc-list :lang=jp` : return a list of installed fonts that support the language based on its locale code
- `fc-list :charset=f303` : return a list of installed fonts that contain the glyph specified by its Unicode code-point

## Links
- More information: https://manned.org/fc-list
- tldr page: https://tldr.inbrowser.app/pages/common/fc-list
