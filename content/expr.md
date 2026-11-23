# expr

Evaluate expressions and manipulate strings.

## Usage
- `expr length "string"` : get the length of a specific string
- `expr substr "string" from length` : get the substring of a string with a specific length
- `expr match "string" 'pattern'` : match a specific substring against an anchored pattern
- `expr index "string" "chars"` : get the first char position from a specific set in a string
- `expr expression1 +|-|*|/|% expression2` : calculate a specific mathematic expression
- `expr expression1 \| expression2` : get the first expression if its value is non-zero and not null otherwise get the second one
- `expr expression1 \& expression2` : get the first expression if both expressions are non-zero and not null otherwise get zero

## Categories
text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/expr-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/expr
