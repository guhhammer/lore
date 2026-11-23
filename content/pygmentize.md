# pygmentize

Python-based syntax highlighter.

## Usage
- `pygmentize file.py` : highlight file syntax and print to `stdout` (language is inferred from the file extension)
- `pygmentize -l javascript input_file` : explicitly set the language for syntax highlighting
- `pygmentize -L lexers` : list available lexers (processors for input languages)
- `pygmentize -f html -o output_file.html input_file.py` : save output to a file in HTML format
- `pygmentize -L formatters` : list available output formats
- `pygmentize -f html -O "full,linenos=True" -o output_file.html input_file` : output an HTML file, with additional formatter options (full page, with line numbers)

## Categories
files

## Links
- More information: https://pygments.org/docs/cmdline/
- tldr page: https://tldr.inbrowser.app/pages/common/pygmentize
