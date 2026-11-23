# xidel

Download and extract data from HTML/XML pages as well as JSON APIs.

## Usage
- `xidel https://www.google.com/search?q=test -e "//a/extract(@href, 'url[?]q=([^&]+)&', 1)[. != '']"` : print all URLs found by a Google search
- `xidel https://www.google.com/search?q=test -f "//a/extract(@href, 'url[?]q=([^&]+)&', 1)[. != '']" -e //title --download '{$host}/'` : print the title of all pages found by a Google search and download them
- `xidel https://example.org -f //a -e //title` : follow all links on a page and print the titles, with XPath
- `xidel https://example.org -f "css('a')" --css title` : follow all links on a page and print the titles, with CSS selectors
- `xidel https://example.org -f "<a>{.}</a>*" -e "<title>{.}</title>"` : follow all links on a page and print the titles, with pattern matching
- `xidel path/to/example.xml -e "<x><foo>ood</foo><bar>{.}</bar></x>"` : read the pattern from `example.xml` (which will also check if the element containing "ood" is there, and fail otherwise)
- `xidel http://stackoverflow.com/feeds -e "<entry><title>{title:=.}</title><link>{uri:=@href}</link></entry>+"` : print all newest Stack Overflow questions with title and URL using pattern matching on their RSS feed
- `xidel https://reddit.com -f "form(css('form.login-form')[1], {'user': '$your_username', 'passwd': '$your_password'})" -e "css('#mail')/@title"` : check for unread Reddit mail, Webscraping, combining CSS, XPath, JSONiq, and automatically form evaluation

## Categories
network, text, documents

## Links
- More information: https://www.videlibri.de/xidel.html#examples
- tldr page: https://tldr.inbrowser.app/pages/common/xidel
