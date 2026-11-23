# prqlc

PRQL compiler. PRQL is a modern language for transforming data - a simple, powerful, pipelined SQL replacement.

## Usage
- `prqlc compile` : run the compiler interactively
- `prqlc compile path/to/file.prql` : compile a specific `.prql` file to `stdout`
- `prqlc compile path/to/source.prql path/to/target.sql` : compile a `.prql` file to a `.sql` file
- `echo "from employees | filter has_dog | select salary" | prqlc compile` : compile a query
- `prqlc watch path/to/directory` : watch a directory and compile on file modification

## Categories
development, files, databases

## Links
- More information: https://prql-lang.org/book/project/integrations/prqlc-cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/prqlc
