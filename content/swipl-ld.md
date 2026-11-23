# swipl-ld

Create a SWI-Prolog embedded executable.

## Usage
- `swipl-ld options c-files o-files pl-files` : the general form, as the manual gives it
- `swipl-ld -pl prolog` : specifies the prolog version to use
- `swipl-ld -help` : shows options briefly
- `swipl-ld -nostate` : just relinks the kernel
- `swipl-ld -c` : compile C or C++ source-files into object files
