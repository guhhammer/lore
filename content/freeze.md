# freeze

Generate images of code and terminal output. Supported formats are PNG, SVG, and WebP.

## Usage
- `freeze path/to/file` : generate an image of code based on a file
- `freeze path/to/file -o path/to/output_image.png` : specify the output path
- `freeze -x command` : generate an image of terminal output
- `freeze path/to/file -i` : interactively customize the output image
- `freeze path/to/file -t dracula` : select a theme for syntax highlighting
- `freeze path/to/file -c base|full|user` : use a base configuration template
- `freeze path/to/file --lines start,end` : capture a specific range of line numbers
- `freeze path/to/file --show-line-numbers` : show line numbers

## See also
- silicon

## Categories
files, containers, media

## Links
- More information: https://github.com/charmbracelet/freeze#flags
- tldr page: https://tldr.inbrowser.app/pages/common/freeze
