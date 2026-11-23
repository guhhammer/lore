# ckbcomp

Compile a XKB keyboard description to a keymap suitable for loadkeys or kbdcontrol.

## Usage
- `ckbcomp [OPTION...] [XKBLAYOUT [XKBVARIANT [XKBOPTIONS]...]]` : the general form, as the manual gives it
- `ckbcomp -v level` : set level of detail for listing
- `ckbcomp -freebsd` : generate a keymap for FreeBSD
- `ckbcomp -backspace [bs|del]` : specifies the behaviour of the <BackSpace> and <Delete> keys
- `ckbcomp -symbols name` : specifies the symbols component name of the XKB keyboard description

## Categories
hardware, development
