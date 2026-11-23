# babeljs-node

CLI that works exactly the same as the Node.js CLI, with the added benefit of compiling with Babel presets and plugins before running it.

## Usage
- `babel-node [options] [ -e script | script.js ] [arguments]` : the general form, as the manual gives it
- `babeljs-node -e` : evaluate script
- `babeljs-node --no-babelrc` : specify whether or not to use .babelrc and .babelignore files
- `babeljs-node -r` : require module
- `babeljs-node -p` : evaluate script and print result

## Categories
development
