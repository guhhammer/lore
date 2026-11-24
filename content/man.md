# man

Format and display manual pages.

## Usage
- `man -Hbrowser_name command` : open the man page for a command in a browser (`=browser_name` can be omitted if `$BROWSER` is set)
- `man 7 command` : display the man page for a command from section 7
- `man -f command` : list all available sections for a command
- `man -w command` : display the location of a manpage rather than the manpage itself
- `man -L locale command` : display the man page using a specific locale
- `man -k "search_string"` : search for manpages containing a search string

## See also
- whatis
- apropos

## Categories
documents, shell

## Links
- More information: https://manned.org/man
- tldr page: https://tldr.inbrowser.app/pages/common/man
- Wikipedia: https://en.wikipedia.org/wiki/Man_page
