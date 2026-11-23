# onionsearch

Scrape URLs on different `.onion` search engines. Note: `onionsearch` requires a Tor proxy running on `localhost:9050`; a Tor enabled browser is needed to visit the `.onion` websites.

## Usage
- `onionsearch "string"` : request results from all the search engines
- `onionsearch "string" --engines tor66|deeplink|phobos|...` : request search results from specific search engines
- `onionsearch "string" --exclude candle|ahmia|...` : exclude certain search engines when searching
- `onionsearch "stuxnet" --engines tor66|deeplink|phobos|... --limit 3` : limit the number of pages to load per engine
- `onionsearch --help | grep -A 1 -i "supported engines"` : list all supported search engines

## Categories
network, text

## Links
- More information: https://github.com/megadose/OnionSearch#--usage
- tldr page: https://tldr.inbrowser.app/pages/common/onionsearch
