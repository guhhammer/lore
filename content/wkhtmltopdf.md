# wkhtmltopdf

Convert HTML documents or web pages into PDF files.

## Usage
- `wkhtmltopdf input.html output.pdf` : convert an HTML document into PDF
- `wkhtmltopdf --page-size A4 input.html output.pdf` : specify the PDF page size (please see `PaperSize` of `QPrinter` for supported sizes)
- `wkhtmltopdf --margin-top|bottom|left|right 10mm input.html output.pdf` : set the PDF page margins
- `wkhtmltopdf --orientation Landscape|Portrait input.html output.pdf` : set the PDF page orientation
- `wkhtmltopdf --grayscale input.html output.pdf` : generate a greyscale version of the PDF document

## Categories
documents, media

## Links
- More information: https://wkhtmltopdf.org/usage/wkhtmltopdf.txt
- tldr page: https://tldr.inbrowser.app/pages/common/wkhtmltopdf
