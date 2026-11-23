# ia

Tool to interact with `archive.org`.

## Usage
- `ia configure` : configure `ia` with API keys (some functions won't work without this step)
- `ia upload identifier path/to/file --metadata="mediatype:data" --metadata="title:example"` : upload one or more items to `archive.org`
- `ia download item` : download one or more items from `archive.org`
- `ia delete identifier file` : delete one or more items from `archive.org`
- `ia search 'subject:"subject" collection:collection'` : search on `archive.org`, returning results as JSON

## Categories
archives, network, files

## Links
- More information: https://archive.org/developers/internetarchive/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/ia
