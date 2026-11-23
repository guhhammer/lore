# samply

Sample and record CPU profiles for analysis.

## Usage
- `samply record path/to/command arg1 arg2 ...` : record a profile of a command and open it in the browser
- `samply record --rate rate_in_hz path/to/command arg1 arg2 ...` : adjust sampling rate
- `samply record --save-only --output path/to/profile.json -- path/to/command arg1 arg2 ...` : record a profile to a file without opening the browser
- `samply load path/to/profile.json` : load a previously saved profile in the browser

## Categories
media, files, hardware

## Links
- More information: https://github.com/mstange/samply
- tldr page: https://tldr.inbrowser.app/pages/common/samply
