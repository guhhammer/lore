# boxes

Draw, remove, and repair ASCII art boxes.

## Usage
- `echo "string" | boxes` : draw a box around a string
- `echo "string" | boxes -r` : remove a box from a string
- `echo "string" | boxes -d parchment` : specify the box design
- `echo "string" | boxes -s 10x5` : specify the box size (in columns by lines)
- `echo "string" | boxes -a hl|c|r` : align the box text horizontally (at left, center, or right)
- `echo "string" | boxes -a vt|c|b` : align the box text vertically (at top, center, or bottom)
- `echo "string" | boxes -a jl|c|rvt` : justify the box text (at left, center, or right)

## Categories
text

## Links
- More information: https://boxes.thomasjensen.com/boxes-man-1.html
- tldr page: https://tldr.inbrowser.app/pages/common/boxes
