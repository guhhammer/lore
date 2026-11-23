# jello

A JSON processor using Python syntax.

## Usage
- `cat file.json | jello` : pretty-print JSON or JSON-Lines data from `stdin` to `stdout`
- `cat file.json | jello -s` : output a schema of JSON or JSON Lines data from `stdin` to `stdout` (useful for grep)
- `cat file.json | jello -l` : output all elements from arrays (or all the values from objects) in JSON or JSON-Lines data from `stdin` to `stdout`
- `cat file.json | jello _[0]` : output the first element in JSON or JSON-Lines data from `stdin` to `stdout`
- `cat file.json | jello '[i.key_name for i in _]'` : output the value of a given key of each element in JSON or JSON-Lines data from `stdin` to `stdout`
- `cat file.json | jello '{"key1": _.key_name1, "key2": _.key_name2, ...}'` : output the value of multiple keys as a new JSON object (assuming the input JSON has the keys `key_name1` and `key_name2`)
- `cat file.json | jello -r '"some text: " + _.key_name'` : output the value of a given key to a string (and disable JSON output)

## Categories
text, files

## Links
- More information: https://github.com/kellyjonbrazil/jello#usage
- tldr page: https://tldr.inbrowser.app/pages/common/jello
