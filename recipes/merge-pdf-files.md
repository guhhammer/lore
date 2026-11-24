# merge pdf files

Join several PDF files into one, and take pages out of one.

## Steps
- `pdfunite a.pdf b.pdf c.pdf all.pdf` : join them in that order (poppler-utils)
- `qpdf --empty --pages a.pdf b.pdf -- all.pdf` : the same with qpdf
- `qpdf in.pdf --pages . 1-3 -- first.pdf` : keep only pages 1 to 3
- `gs -dBATCH -dNOPAUSE -q -sDEVICE=pdfwrite -sOutputFile=all.pdf a.pdf b.pdf` : with Ghostscript, which also shrinks them

## See also
- pdfunite
- qpdf
- gs

## Categories
files
