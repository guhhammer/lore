# kmscon

Use the framebuffer instead of text mode to draw a terminal in a TTY.

## Usage
- `sudo kmscon` : start `kmscon` on the first available TTY, as reported by `fgconsole --next-available`
- `sudo kmscon --vt /dev/ttyX|ttyX|X` : start `kmscon` in a specific TTY
- `sudo kmscon --mouse` : enable mouse support
- `sudo kmscon -l command` : specify the command to use for login
- `systemctl enable --now kmsconvt@ttyX` : always start `kmscon` in a specific terminal

## Categories
shell, users, text

## Links
- More information: https://manned.org/kmscon
- tldr page: https://tldr.inbrowser.app/pages/linux/kmscon
- Wikipedia: https://en.wikipedia.org/wiki/Kmscon
