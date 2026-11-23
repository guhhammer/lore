# ghcid

Simple and efficient CLI IDE for Haskell that reloads code on file changes. Continuously displays compile errors, warnings, and test results.

## Usage
- `ghcid path/to/Main.hs` : start `ghcid` and monitor a Haskell file for changes
- `ghcid --command "stack ghci Main.hs"` : start `ghcid` with a specific command, such as loading a Stack or Cabal project
- `ghcid --run=action path/to/Main.hs` : run an action (default `main`) on each file save
- `ghcid --height=height --width=width path/to/Main.hs` : set maximum height and width (default to console height and width)
- `ghcid --outputfile=path/to/output_file.txt path/to/Main.hs` : write full GHC compiler output to a file
- `ghcid --allow-eval path/to/Main.hs` : execute REPL commands (eg. `-- $> 1+1`) on each file save

## Categories
files, development

## Links
- More information: https://github.com/ndmitchell/ghcid
- tldr page: https://tldr.inbrowser.app/pages/common/ghcid
