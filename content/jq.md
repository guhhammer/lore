# jq

A JSON processor that uses a domain-specific language (DSL).

## Usage
- `jq '.' path/to/file.json` : execute a specific expression only using the `jq` binary (print a colored and formatted JSON output)
- `cat path/to/file.json | jq -f path/to/script.jq` : execute a specific script
- `cat path/to/file.json | jq --arg "name1" "value1" --arg "name2" "value2" ... '. + $ARGS.named'` : pass specific arguments
- `cat path/to/multiple_json_file_*.json | jq '{newKey1: .key1, newKey2: .key2.nestedKey, ...}'` : create new JSON object via old JSON objects from multiple files
- `cat path/to/file.json | jq '.[index1], .[index2], ...'` : print specific array items
- `cat path/to/file.json | jq '.[]'` : print all array/object values
- `cat path/to/file.json | jq '.[] | select((.key1=="value1") and .key2=="value2")'` : print objects with 2-condition filter in array
- `cat path/to/file.json | jq '. +|- {"key1": "value1", "key2": "value2", ...}'` : add/remove specific keys

## Categories
text

## Links
- More information: https://jqlang.org/manual/
- tldr page: https://tldr.inbrowser.app/pages/common/jq
