# emulate

Reset shell options to match a certain shell's behavior. Also used to ensure a shell function runs with the default configuration.

## Usage
- `emulate` : print current emulation mode
- `emulate bash` : adjust settings of the current shell session to mimic Bash
- `emulate ksh -c "echo hi"` : evaluate a command within temporary emulation
- `emulate zsh -R` : emulate the default Zsh, Resetting options when possible
- `my_function() { emulate -L zsh|sh|ksh|csh; ls }` : scope emulation to the current function

## Categories
shell

## Links
- More information: https://github.com/antonio/zsh-config/blob/master/help/emulate
- tldr page: https://tldr.inbrowser.app/pages/common/emulate
