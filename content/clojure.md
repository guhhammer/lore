# clojure

This command is an alias of `clj`.

## Usage
- `clj` : start a REPL (interactive shell)
- `clj -X namespace/function_name` : execute a function
- `clj -M -m namespace args` : run the main function of a specified namespace
- `clj -P` : prepare a project by resolving dependencies, downloading libraries, and making/caching classpaths
- `clj -Sdeps '{:deps {nrepl {:mvn/version "0.7.0"} cider/cider-nrepl {:mvn/version "0.25.2"}\}\}' -m nrepl.cmdline --middleware '["cider.nrepl/cider-middleware"]' --interactive` : start an nREPL server with the CIDER middleware
- `clj -Sdeps '{:deps {org.clojure/clojurescript {:mvn/version "1.10.758"}\}\}' -m cljs.main -r` : start a REPL for ClojureScript and open a web browser

## Categories
shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/clojure
