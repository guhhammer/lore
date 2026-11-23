# mmv

Move and rename files in bulk.

## Usage
- `mmv "*.old_extension" "#1.new_extension"` : rename all files with a certain extension to a different extension
- `mmv -c "report*part*.txt" "./french/rapport#1partie#2.txt"` : copy `report6part4.txt` to `./french/rapport6partie4.txt` along with all similarly named files
- `mmv -a "*.txt" "all.txt"` : append all `.txt` files into one file
- `mmv "[0-1][0-9]-[0-3][0-9]-[0-9][0-9][0-9][0-9].txt" "#3#4-#1#2-#5#6#7#8.txt"` : convert dates in filenames from "M-D-Y" format to "D-M-Y" format

## Categories
files

## Links
- More information: https://manned.org/mmv.1
- tldr page: https://tldr.inbrowser.app/pages/common/mmv
