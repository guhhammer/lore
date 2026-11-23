# zle

Manage Zsh Line Editor widgets. Note: Some operations require ZLE to be active (typically inside a user-defined widget).

## Usage
- `zle -l` : list user-defined widgets
- `zle -la` : list all widgets, including built-in ones
- `zle -N widget_name optional_shell_function_name` : create a New user-defined widget (function defaults to same name as widget)
- `zle -D widget_name` : delete a widget
- `zle -A original_widget alias_name` : create an Alias of a widget
- `zle widget_name` : invoke a widget from inside another widget function (requires active ZLE)
- `zle -U "text"` : push text into ZLE's input queue as if it were typed (requires active ZLE)

## See also
- bindkey

## Categories
text, users, editors, shell

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Zsh-Line-Editor.html#Zle-Builtins
- tldr page: https://tldr.inbrowser.app/pages/common/zle
