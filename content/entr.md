# entr

Run arbitrary commands when files change.

## Usage
- `ag --files-with-matches | entr make` : rebuild with `make` if any file in any subdirectory changes
- `ls *.c | entr 'make && make test'` : rebuild and test with `make` if any `.c` source files in the current directory change
- `ls *.rb | entr -r ruby main.rb` : send a `SIGTERM` to any previously spawned ruby subprocesses before executing `ruby main.rb`
- `ls *.sql | entr psql -f /_` : run a command with the changed file (`/_`) as an argument
- `echo my.sql | entr -cp psql -f /_` : clear the screen and run a query after the SQL script is updated
- `find src/ | entr -s 'make | sed 10q'` : rebuild the project if source files change, limiting output to the first few lines
- `ls *.js | entr -r node app.js` : launch and auto-reload a Node.js server

## See also
- watchexec

## Categories
databases, development, files

## Links
- More information: https://eradman.com/entrproject/
- tldr page: https://tldr.inbrowser.app/pages/common/entr
