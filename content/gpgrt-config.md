# gpgrt-config

Helper script to get information about the installed version of gpg libraries.

## Usage
- `gpgrt-config [options] [ module ]` : the general form, as the manual gives it
- `gpgrt-config --modversion` : print the currently installed version of module on the standard output
- `gpgrt-config --static` : modifies output if linking statically instead of dynamically
- `gpgrt-config --variable=VARNAME` : output variable VARNAME from the module's pkg-config file

## Categories
development, security, system
