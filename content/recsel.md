# recsel

Print records from a recfile: a human-editable, plain text database.

## Usage
- `recsel -p name,version data.rec` : extract name and version field
- `recsel -e "field_name ~ 'regex' data.rec"` : use "~" to match a string with a given `regex`
- `recsel -e "name ~ 'regex' && version ~ 'regex'" data.rec` : use a predicate to match a name and a version

## Categories
text, databases, documents

## Links
- More information: https://www.gnu.org/software/recutils/manual/recutils.html#Invoking-recsel
- tldr page: https://tldr.inbrowser.app/pages/common/recsel
