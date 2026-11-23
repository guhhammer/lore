# expr

Evalueer expressies en manipuleer strings.

## Usage
- `expr length "string"` : krijg de lengte van een specifieke string
- `expr substr "string" van lengte` : krijg de substring van een string met een specifieke lengte
- `expr match "string" 'patroon'` : vergelijk een specifieke substring met een verankerd patroon
- `expr index "string" "karakters"` : verkrijg de eerste karakterpositie van een specifieke set in een tekenreeks
- `expr expressie1 +|-|*|/|% expressie2` : bereken een specifieke mathematische expressie
- `expr expressie1 \| expressie2` : krijg de eerste expressie als de waarde niet nul is en niet null, anders de tweede
- `expr expressie1 \& expressie2` : krijg de eerste expressie als beide expressies niet nul zijn en niet null, anders 0

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/expr-invocation.html
