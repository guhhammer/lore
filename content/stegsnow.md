# stegsnow

Steganography tool for concealing and extracting messages in text files encoded as tabs and spaces.

## Usage
- `stegsnow path/to/file.txt` : extract message from file
- `stegsnow -C -p password path/to/file.txt` : extract Compressed and password protected message from file
- `stegsnow -S -l 72 path/to/file.txt` : determine approximate Storage capacity with line length less than 72 for file
- `stegsnow -m 'message' path/to/file.txt path/to/result.txt` : conceal message in text from file and save to result
- `stegsnow -C -f 'path/to/message.txt' path/to/file.txt path/to/result.txt` : conceal message file content Compressed in text from file and save to result
- `stegsnow -C -p password -m 'message' path/to/file.txt path/to/result.txt` : conceal message Compressed and password protected in text from file and save to result

## Categories
files, text, users

## Links
- More information: https://darkside.com.au/snow/manual.html
- tldr page: https://tldr.inbrowser.app/pages/linux/stegsnow
