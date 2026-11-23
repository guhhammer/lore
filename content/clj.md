# clj

Clojure tool to start a REPL or invoke a function with data. All options can be defined in a `deps.edn` file.

## Usage
- `clj` : start a REPL (interactive shell)
- `clj -X namespace/function_name` : execute a function
- `clj -M -m namespace args` : run the main function of a specified namespace
- `clj -P` : prepare a project by resolving dependencies, downloading libraries, and making/caching classpaths
- `clj -Sdeps '{:deps {nrepl {:mvn/version "0.7.0"} cider/cider-nrepl {:mvn/version "0.25.2"}\}\}' -m nrepl.cmdline --middleware '["cider.nrepl/cider-middleware"]' --interactive` : start an nREPL server with the CIDER middleware
- `clj -Sdeps '{:deps {org.clojure/clojurescript {:mvn/version "1.10.758"}\}\}' -m cljs.main -r` : start a REPL for ClojureScript and open a web browser

## Categories
files

## Links
- More information: https://clojure.org/guides/deps_and_cli
- tldr page: https://tldr.inbrowser.app/pages/common/clj
