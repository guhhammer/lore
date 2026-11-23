# winicontoppm

This command has been superseded by `winicontopam`.

## Usage
- `winicontopam path/to/input_file.ico >path/to/output.pam` : read an ICO file and convert the best quality image contained therein to the PAM format
- `winicontopam -al path/to/input_file.ico >path/to/output.pam` : convert all images in the input file to PAM
- `winicontopam -i n path/to/input_file.ico >path/to/output.pam` : convert the n'th image in the input file to PAM
- `winicontopam -an path/to/input_file.ico >path/to/output.pam` : if the image(s) to be extracted contain graded transparency data and an AND mask, write the AND mask into the fifth channel of the output PAM file

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/winicontoppm.html
- tldr page: https://tldr.inbrowser.app/pages/common/winicontoppm
