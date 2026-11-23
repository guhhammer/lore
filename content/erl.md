# erl

Run and manage programs in the Erlang programming language.

## Usage
- `erlc path/to/file1 path/to/file2 ... && erl -noshell 'mymodule:myfunction(arguments), init:stop().'` : compile and run sequential Erlang program as a common script and then exit
- `erl -remsh nodename@hostname -sname custom_shortname -hidden -setcookie cookie_of_remote_node` : connect to a running Erlang node
- `erl -pa path/to/directory_with_beam_files` : tell the Erlang shell to load modules from a directory

## Categories
development, files

## Links
- More information: https://erlang.org/documentation/doc-16.0/erts-16.0/doc/html/erl_cmd.html
- tldr page: https://tldr.inbrowser.app/pages/common/erl
