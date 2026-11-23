# xe

Execute a command once for each line piped from another command or file.

## Usage
- `arguments_source | xe command` : run a command once for each line of input data as arguments
- `arguments_source | xe command {} optional_extra_arguments` : execute the commands, replacing any occurrence of the placeholder (marked as `{}`) with the input line
- `echo -e 'a\nb' | xe -N2 -s 'echo $2 $1'` : execute a shellscript, joining every `N` lines into a single call
- `find . -name '*.backup' | xe rm -v` : delete all files with a `.backup` extension
- `arguments_source | xe -j max_jobs command` : run up to `max-jobs` processes in parallel; the default is 1. If `max-jobs` is 0, xe will run as many processes as cpu cores

## Categories
text, processes, files

## Links
- More information: https://github.com/leahneukirchen/xe/blob/master/README
- tldr page: https://tldr.inbrowser.app/pages/common/xe
