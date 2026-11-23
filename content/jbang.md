# jbang

Easily create, edit, and run self-contained source-only Java programs.

## Usage
- `jbang init path/to/file.java` : initialize a simple Java class
- `jbang init -t=cli path/to/file.java` : initialize a Java class (useful for scripting)
- `jbang run -i` : use `jshell` to explore and use a script and any dependencies in a REPL editor
- `jbang edit --open=codium|code|eclipse|idea|netbeans|gitpod path/to/script.java` : set up a temporary project to edit a script in an IDE
- `echo 'Files.list(Paths.get("/etc")).forEach(System.out::println);' | jbang -` : run a Java code snippet (Java 9 and later)
- `jbang path/to/file.java command arg1 arg2 ...` : run command-line application
- `jbang app install --name command_name path/to/script.java` : install a script on the user's `$PATH`
- `jbang jdk install version` : install a specific version of JDK to be used with `jbang`

## See also
- java

## Categories
files, development, packages

## Links
- More information: https://www.jbang.dev/documentation/jbang/latest/cli/jbang.html
- tldr page: https://tldr.inbrowser.app/pages/common/jbang
