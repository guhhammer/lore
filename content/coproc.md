# coproc

Bash builtin for creating interactive asynchronous subshells.

## Usage
- `coproc { command1; command2; ...; }` : run a subshell asynchronously
- `coproc name { command1; command2; ...; }` : create a coprocess with a specific name
- `echo "input" >&"${name[1]}"` : write to a specific coprocess `stdin`
- `read <&"${name[0]}" variable` : read from a specific coprocess `stdout`
- `coproc name { while read line; do command1; command2; ...; done }` : create a coprocess which repeatedly reads `stdin` and runs some commands on the input
- `coproc name { while read line; do echo "$line" | command1 | command2 | ... | cat /dev/fd/0; done }` : create a coprocess which repeatedly reads `stdin`, runs a pipeline on the input, and writes the output to `stdout`
- `coproc BC { bc -l; }; echo "1/3" >&"${BC[1]}"; read <&"${BC[0]}" output; echo "$output"` : create and use a coprocess running `bc`

## Categories
shell, text

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Coprocesses
- tldr page: https://tldr.inbrowser.app/pages/common/coproc
