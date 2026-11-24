# scrapy

Web-crawling framework.

## Usage
- `scrapy startproject project_name` : create a project
- `scrapy genspider spider_name website_domain` : create a spider (in project directory)
- `scrapy edit spider_name` : edit spider (in project directory)
- `scrapy crawl spider_name` : run spider (in project directory)
- `scrapy fetch url` : fetch a webpage as Scrapy sees it and print the source to `stdout`
- `scrapy view url` : open a webpage in the default browser as Scrapy sees it (disable JavaScript for extra fidelity)
- `scrapy shell url` : open Scrapy shell for URL, which allows interaction with the page source in a Python shell (or IPython if available)

## Categories
network, development, files

## Links
- More information: https://docs.scrapy.org/en/latest/topics/commands.html#using-the-scrapy-tool
- tldr page: https://tldr.inbrowser.app/pages/common/scrapy
- Wikipedia: https://en.wikipedia.org/wiki/Scrapy
