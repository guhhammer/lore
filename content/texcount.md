# texcount

Count words in TeX documents omitting macros. Note: If the TeX document uses `\include` or `\input` and you want to count the included files, `texcount` must be run in the directory of the root TeX file.

## Usage
- `texcount path/to/file.tex` : count words in a TeX file
- `texcount -merge file.tex` : count words in a document and subdocuments built with `\input` or `\include`
- `texcount -inc file.tex` : count words in a document and subdocuments, listing each file separately (and a total count)
- `texcount -merge -sub=chapter file.tex` : count words in a document and subdocuments, producing subcounts by chapter (instead of subsection)
- `texcount -v path/to/file.tex` : count words with verbose output

## Categories
files, documents

## Links
- More information: https://app.uio.no/ifi/texcount/howto.html
- tldr page: https://tldr.inbrowser.app/pages/common/texcount
