# kiterunner wordlist

A contextual web scanner for managing wordlists used in API and web endpoint discovery. The `wordlist` subcommand handles listing and saving wordlists in `~/.cache/kiterunner`.

## Usage
- `kiterunner wordlist list` : list all cached and available Assetnote wordlists
- `kiterunner wordlist list -o json` : list wordlists with JSON output
- `kiterunner wordlist list -v debug` : list wordlists with verbose debug output
- `kiterunner wordlist save apiroutes-210328` : save a specific Assetnote wordlist by alias
- `kiterunner wordlist save path/to/httparchive_apiroutes_2024_05_28.txt` : save a specific Assetnote wordlist by full filename
- `kiterunner wordlist save apiroutes-210328,aspx-210328` : save multiple wordlists by alias
- `kiterunner wordlist save apiroutes-210328 -q` : save a wordlist with quiet mode to suppress output

## Links
- More information: https://github.com/assetnote/kiterunner#usage
- tldr page: https://tldr.inbrowser.app/pages/common/kiterunner-wordlist
