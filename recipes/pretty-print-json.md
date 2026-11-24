# pretty-print json

Show JSON indented and coloured, pick values out of it, from a file or a web request.

## Steps
- `jq . file.json` : the file, indented and coloured
- `curl -s https://api.example.com/items | jq .` : a web answer, the same way
- `jq '.items[0].name' file.json` : one value out of it
- `jq -r '.items[].name' file.json` : a value from every item, as plain text
- `python3 -m json.tool file.json` : without jq: indented by Python

## Notes
- jq is a package of its own on most systems: `sudo apt install jq`, `brew install jq`.

## See also
- jq
- curl
- python3

## Categories
text development
