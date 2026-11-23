# pdfjam

Shell frontend for the LaTeX pdfpages package for mingling PDFs.

## Usage
- `pdfjam path/to/file1.pdf path/to/file2.pdf ... -o path/to/output_file.pdf` : merge two (or more) PDFs
- `pdfjam path/to/file1.pdf 1 path/to/file2.pdf 1 ... -o path/to/output_file.pdf` : merge the first page of each file together
- `pdfjam path/to/file1.pdf 3-5,1 path/to/file2.pdf 4-6 -o path/to/output_file.pdf` : merge subranges from two PDFs
- `pdfjam path/to/file.pdf path/to/signature --fitpaper true -o path/to/signed.pdf --nup "1x2" --delta "0 -842pt"` : sign an A4 page (adjust delta to height for other formats) with a scanned signature by overlaying them
- `pdfjam path/to/file.pdf --nup 2x2 --suffix 4up --preamble '\usepackage{fancyhdr} \pagestyle{fancy}'` : arrange the pages from the input file into a fancy 2x2 grid
- `pdfjam path/to/file1.pdf last-1 path/to/file2.pdf last-1 ... --suffix reversed` : reverse the order of pages within each given file and concatenate them

## Categories
files, documents, media

## Links
- More information: https://github.com/pdfjam/pdfjam/blob/master/doc/pdfjam-help.txt
- tldr page: https://tldr.inbrowser.app/pages/common/pdfjam
