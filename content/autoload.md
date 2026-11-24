# autoload

Mark functions for lazy loading in Zsh. Functions are not loaded into memory until first called, improving shell startup time.

## Usage
- `autoload function_name` : autoload a function by name
- `autoload +X function_name` : autoload a function and immediately resolve its definition
- `autoload -Uz function_name` : autoload a function using Zsh-style autoloading (recommended)
- `fpath=(path/to/functions_dir $fpath) && autoload -Uz function_name` : make functions from a directory available by adding it to `fpath`
- `autoload -Uz compinit && compinit` : autoload the Zsh completion system
- `autoload -Uz add-zsh-hook` : autoload and use the `add-zsh-hook` utility

## Categories
shell, monitoring

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Shell-Builtin-Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/autoload
- Wikipedia: https://en.wikipedia.org/wiki/Autoload
