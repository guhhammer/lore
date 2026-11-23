# choose

A human-friendly and fast alternative to `cut` and (sometimes) `awk`.

## Usage
- `choose 4` : print the 5th item from a line (starting from 0)
- `choose -f ':' 0 2 4` : print the first, 3rd, and 5th item from a line, where items are separated by ':' instead of whitespace
- `choose 1:4` : print everything from the 2nd to 5th item on the line, including the 5th
- `choose -x 1:4` : print everything from the 2nd to 5th item on the line, excluding the 5th
- `choose :2` : print the beginning of the line to the 3rd item
- `choose -x :2` : print all items from the beginning of the line until the 3rd item (exclusive)
- `choose 2:` : print all items from the 3rd to the end of the line
- `choose -1` : print the last item from a line

## Categories
documents, text

## Links
- More information: https://github.com/theryangeary/choose#usage
- tldr page: https://tldr.inbrowser.app/pages/common/choose
