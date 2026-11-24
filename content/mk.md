# mk

Task runner for targets described in Mkfile. Mostly used to control the compilation of an executable from source code.

## Usage
- `mk` : call the first target specified in the Mkfile (usually named "all")
- `mk target` : call a specific target
- `NPROC=4 mk target` : call a specific target, executing 4 jobs at a time in parallel
- `mk -wtarget target` : force mking of a target, even if source files are unchanged
- `mk -a target` : assume all targets to be out of date. Thus, update `target` and all of its dependencies
- `mk -k` : keep going as far as possible on error

## Categories
development

## Links
- More information: https://doc.cat-v.org/plan_9/4th_edition/papers/mk
- tldr page: https://tldr.inbrowser.app/pages/common/mk
- Wikipedia: https://en.wikipedia.org/wiki/Make_(software)
