# ed

The original Unix text editor.

## Usage
- `ed` : start an interactive editor session with an empty document
- `ed -p '> '` : start an interactive editor session with an empty document and a specific prompt
- `ed -v` : start an interactive editor session with user-friendly errors
- `ed -q -s` : start an interactive editor session with an empty document and without diagnostics, byte counts, and '!' prompt
- `ed -l` : start an interactive editor session without exit status change when command fails
- `ed path/to/file` : edit a specific file (this shows the byte count of the loaded file)
- `,s/regex/replacement/g` : [Interactive] Replace a string with a specific replacement for all lines
- `q` : [Interactive] Exit `ed`

## See also
- awk
- sed

## Categories
editors, text, files

## Links
- More information: https://www.gnu.org/software/ed/manual/ed_manual.html
- tldr page: https://tldr.inbrowser.app/pages/common/ed
