# jless

View and explore JSON data interactively.

## Usage
- `jless path/to/file.json` : open a JSON file
- `cat path/to/file.json | jless` : read JSON from `stdin`
- `jless --yaml path/to/file` : force YAML parsing regardless of file extension
- `jless -m line path/to/file.json` : start in line mode (show braces, commas, and quoted object keys)
- `jless -N path/to/file.json` : hide line numbers
- `jless -r path/to/file.json` : show relative line numbers
- `</>pattern` : [Interactive] Search forward for a pattern (press `<n>`/`<N>` for next/previous match)
- `<q>` : [Interactive] Exit

## See also
- jq
- fx
- less

## Categories
text, files

## Links
- More information: https://jless.io/user-guide
- tldr page: https://tldr.inbrowser.app/pages/common/jless
