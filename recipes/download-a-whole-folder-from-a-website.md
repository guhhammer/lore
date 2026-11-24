# download a whole folder from a website

Fetch every file under a web folder, without climbing to the folders above it.

## Steps
- `wget -r -np -nH -R 'index.html*' https://example.com/files/` : every file under /files/, not the folders above (-np), without index pages
- `wget -m -np https://example.com/docs/` : mirror it, keeping the dates, to update later
- `wget -c https://example.com/big.iso` : a single file, resuming if it was cut
- `wget -i urls.txt` : every address listed in a file

## Notes
- macOS has no wget: `brew install wget`, or `curl -O` for one file.

## See also
- wget
- curl

## Categories
network files
