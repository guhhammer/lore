# clj

Clojure-tool om een REPL te starten of een functie met data aan te roepen. Alle opties kunnen worden gedefinieerd in een `deps.edn` bestand.

## Usage
- `clj` : start een REPL (interactieve shell)
- `clj -X namespace/functie_naam` : voer een functie uit
- `clj -M -m namespace args` : voer de voornaamste functie uit van een gespecificeerde namespace
- `clj -P` : bereid een project voor door afhankelijkheden op te lossen, het downloaden van bibliotheken en het maken/cachen van classpaths
- `clj -Sdeps '{:deps {nrepl {:mvn/version "0.7.0"} cider/cider-nrepl {:mvn/version "0.25.2"}\}\}' -m nrepl.cmdline --middleware '["cider.nrepl/cider-middleware"]' --interactive` : start een nREPL server met de CIDER middleware
- `clj -Sdeps '{:deps {org.clojure/clojurescript {:mvn/version "1.10.758"}\}\}' -m cljs.main -r` : start een REPL voor ClojureScript en open een web browser

## Links
- More information: https://clojure.org/guides/deps_and_cli
