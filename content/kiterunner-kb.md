# kiterunner kb

A contextual web scanner for manipulating kitebuilder schemas used in API and web endpoint discovery. The `kb` subcommand handles schema compilation, conversion, parsing, and request replay.

## Usage
- `kiterunner kb compile path/to/wordlist.json path/to/wordlist.kite` : compile a kitebuilder schema from JSON to a kite file
- `kiterunner kb convert path/to/wordlist.kite path/to/wordlist.txt` : convert a kite file to a text wordlist
- `kiterunner kb convert path/to/wordlist.txt path/to/wordlist.kite` : convert a text wordlist to a kite file
- `kiterunner kb convert path/to/wordlist.kite path/to/wordlist.json` : convert a kite file to a JSON schema
- `kiterunner kb parse path/to/wordlist.json -o json` : parse a kitebuilder schema and output prettified JSON data
- `kiterunner kb parse path/to/wordlist.kite -o text` : parse a kite file and output prettified text data
- `kiterunner kb replay -w path/to/wordlist.kite "request_output"` : replay a specific request from a kitebuilder schema output
- `kiterunner kb replay -w path/to/wordlist.kite -p http://localhost:8080 "request_output"` : replay a request through a proxy for inspection

## Categories
files, text, network

## Links
- More information: https://github.com/assetnote/kiterunner#usage
- tldr page: https://tldr.inbrowser.app/pages/common/kiterunner-kb
