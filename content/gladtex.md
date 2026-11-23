# gladtex

A LaTeX formula preprocessor for HTML files. It converts LaTeX formulas to images.

## Usage
- `gladtex path/to/input.htex` : convert to HTML
- `gladtex path/to/input.htex -o path/to/output.html` : save the converted file to a specific output file
- `gladtex path/to/input.htex -d path/to/image_output_directory` : save the generated images to a specific directory
- `gladtex path/to/input.htex -r resolution` : set image resolution (in dpi, default is 100)
- `gladtex path/to/input.htex -k` : keep LaTeX files after conversion
- `gladtex path/to/input.htex -b background_color -f foreground_color` : set background and foreground color of the images
- `pandoc -s -t html --gladtex path/to/input.md | gladtex -o path/to/output.html` : convert Markdown to HTML using `pandoc` and `gladtex`

## Categories
files, documents, media

## Links
- More information: https://manned.org/gladtex
- tldr page: https://tldr.inbrowser.app/pages/common/gladtex
