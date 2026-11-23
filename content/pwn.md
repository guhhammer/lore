# pwn

Exploit Development Library designed for rapid prototyping.

## Usage
- `pwn asm "xor edi, edi"` : convert the given assembly code to `bytes`
- `pwn cyclic number` : create a cyclic pattern of the specific number of characters
- `pwn hex deafbeef` : encode the given data into the hexadecimal system
- `pwn unhex 6c4f7645` : decode the given data from hexadecimal
- `pwn shellcraft amd64.linux.sh` : print a x64 Linux shellcode for running a shell
- `pwn checksec path/to/file` : check the binary security settings for the given ELF file
- `pwn update` : check for Pwntools updates
- `pwn version` : display version

## Categories
development, files

## Links
- More information: https://docs.pwntools.com/en/stable/commandline.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pwn
